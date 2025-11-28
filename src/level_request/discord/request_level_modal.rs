use crate::config::constants::{EMPTY_STRING, LEVEL_REQUEST_MODAL_REQUEST_VALID_UNTIL};
use crate::level_request::model::level_request::{LevelRequest, UpdateLevelRequestMessageId};
use crate::level_request::service::level_request_service::LevelRequestService;
use crate::send_level::model::request_score::RequestRating;
use crate::serenity::discord::{create_thread, get_verify_level_request_buttons, invoke_component_ephemeral, invoke_modal_ephemeral, log_action_to_discord, log_error_to_discord, send_level_request_message_to_discord};
use crate::serenity::modals::extract_modal_components;
use log::{error, warn};
use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, ModalInteraction};
use serenity::builder::CreateComponent;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use chrono::Utc;
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::interval;
use crate::level_request::model::level_request_modal_request::LevelRequestModalRequest;

static REQUEST_BUFFER: OnceLock<Arc<Mutex<HashMap<u64, LevelRequestModalRequest>>>> = OnceLock::new();

pub async fn run_verify_request(ctx: &Context, modal_interaction: &ModalInteraction) {
	let modal_inputs = extract_modal_components(
		&modal_interaction,
		vec![
			"level-id".to_string(),
			"request-rating".to_string(),
			"video-link".to_string(),
			"request-feedback".to_string(),
			"notify".to_string(),
		],
	);
	let Ok(level_id) = modal_inputs
		.get("level-id")
		.unwrap_or(&EMPTY_STRING)
		.parse::<u64>()
	else {
		return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
	};
	let Ok(request_rating) = modal_inputs
		.get("request-rating")
		.unwrap_or(&EMPTY_STRING)
		.parse::<RequestRating>()
	else {
		return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
	};
	let video_link = modal_inputs.get("video-link").unwrap_or(&EMPTY_STRING);
	let Ok(has_requested_feedback) = modal_inputs
		.get("request-feedback")
		.unwrap_or(&EMPTY_STRING)
		.parse::<bool>()
	else {
		return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
	};
	let Ok(notify) = modal_inputs
		.get("notify")
		.unwrap_or(&EMPTY_STRING)
		.parse::<bool>()
	else {
		return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
	};
	let discord_user_id = modal_interaction.user.id.get();
	let level_request = LevelRequest::new(
		level_id,
		discord_user_id,
		request_rating,
		video_link.to_string(),
		has_requested_feedback,
		notify,
	);
	let level_request_service = LevelRequestService::new();
	let gd_level_info = match level_request_service.get_gd_level_info(level_id).await {
		Ok(Some(gd_level_info)) => gd_level_info,
		Ok(None) => {
			warn!("Level with ID {} does not exist", level_id);
			return invoke_modal_ephemeral("Level does not exist.", &ctx, &modal_interaction).await;
		}
		Err(get_gd_level_info_error) => {
			error!("Unable to get gd level info {}", get_gd_level_info_error);
			return invoke_modal_ephemeral(
				&get_gd_level_info_error.to_string(),
				&ctx,
				&modal_interaction,
			)
			.await;
		}
	};
	let mut request_buffer = get_request_buffer().await;
	request_buffer.insert(discord_user_id, LevelRequestModalRequest::new(
		level_request,
		Utc::now() + LEVEL_REQUEST_MODAL_REQUEST_VALID_UNTIL
	));
	drop(request_buffer);

	let components = &[CreateComponent::ActionRow(CreateActionRow::Buttons(
		Cow::Owned(get_verify_level_request_buttons()),
	))];
	let test = CreateInteractionResponse::Message(
		CreateInteractionResponseMessage::new()
			.ephemeral(true)
			.content(format!(
				"Are you sure you want to request {} ({}) by {}?",
				gd_level_info.level_name, level_id, gd_level_info.level_author
			))
			.components(components),
	);

	if let Err(err) = modal_interaction.create_response(&ctx.http, test).await {
		error!("Cannot respond to modal: {err}");
	};
}

pub async fn run_request_level_modal_button_submit(ctx: &Context, button_interaction: &ComponentInteraction) {
	let discord_user_id = button_interaction.user.id.get();
	let level_request: LevelRequest;
	let mut request_buffer = REQUEST_BUFFER
		.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
		.lock()
		.await;

	level_request = if let Some(level_request_modal_request) = request_buffer.remove(&discord_user_id) {
		level_request_modal_request.level_request
	} else {
		error!("No request in buffer for user {}", discord_user_id);
		return invoke_component_ephemeral("Unable to make level request.", &ctx, &button_interaction)
			.await;
	};
	drop(request_buffer);
	let level_request_service = LevelRequestService::new();

	match level_request_service
		.level_request_service(&level_request)
		.await
	{
		Ok(requested_level) => {
			match send_level_request_message_to_discord(&ctx, &requested_level).await {
				Ok(message_data) => {
					if let Err(create_thread_error) = create_thread(
						&ctx,
						&button_interaction.user,
						message_data.id.get(),
						&requested_level,
					)
					.await
					{
						error!("Error creating thread: {}", create_thread_error);
						log_error_to_discord(
							&button_interaction.user,
							"creating thread for requested level",
							&create_thread_error,
							Some(&requested_level),
							&ctx,
						)
						.await;
					}

					let update_request_message_id = UpdateLevelRequestMessageId {
						level_id: requested_level.level_id,
						discord_message_id: message_data.id.get(),
					};
					if let Err(error) = level_request_service
						.update_request_message_id(update_request_message_id)
						.await
					{
						error!("Error updating message ID: {error:?}");
					}
				}
				Err(send_message_error) => {
					error!("Error sending message: {send_message_error}");
				}
			}
			let modal_ephemeral_content = "Level has been requested successfully!";
			invoke_component_ephemeral(&modal_ephemeral_content, &ctx, &button_interaction).await;

			log_action_to_discord(
				&button_interaction.user,
				"requested a level",
				Some(&requested_level),
				&ctx,
			)
			.await;
		}
		Err(request_level_error) => {
			invoke_component_ephemeral(&request_level_error.to_string(), &ctx, &button_interaction)
				.await;
			log_error_to_discord(
				&button_interaction.user,
				"requesting a level",
				&request_level_error,
				None,
				&ctx,
			)
			.await;
		}
	}
}

pub async fn run_request_level_modal_button_cancel(ctx: &Context, button_interaction: &ComponentInteraction) {
	let discord_user_id = button_interaction.user.id.get();
	let mut request_buffer = REQUEST_BUFFER
		.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
		.lock()
		.await;

	request_buffer.remove(&discord_user_id);
	drop(request_buffer);
	if let Err(acknowledge_error) = button_interaction.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await {
		error!("Cannot respond to request level cancellation: {}", acknowledge_error);
	}
}

pub fn remove_stale_requests() {
	let mut interval = interval(Duration::from_secs(30));

	tokio::spawn(async move {
		loop {
			interval.tick().await;
			let now = Utc::now();
			let mut request_buffer = get_request_buffer().await;
			println!("{:?}", request_buffer);

			request_buffer.retain(
				|_, level_request_modal_request|
					level_request_modal_request.is_valid_request(now)
			);
		}
	});
}

async fn get_request_buffer<'a>() -> MutexGuard<'a, HashMap<u64, LevelRequestModalRequest>> {
	REQUEST_BUFFER
		.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
		.lock()
		.await
}

async fn handle_input_parse_error(
	invalid_field: &str,
	ctx: &Context,
	modal_interaction: &ModalInteraction,
) {
	let modal_ephemeral_content = format!("Invalid input for field: {}", invalid_field);
	invoke_modal_ephemeral(modal_ephemeral_content.as_str(), &ctx, &modal_interaction).await
}