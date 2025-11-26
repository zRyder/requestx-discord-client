use std::str::FromStr;

use log::error;
use serenity::{
	all::{CommandInteraction, CommandOptionType},
	builder::{CreateCommand, CreateCommandOption},
	prelude::Context
};
use serenity::all::{GenericChannelId, MessageId};
use crate::{
	config::client_config::CLIENT_CONFIG,
	level_request::{
		model::level_request::{LevelRequest, UpdateLevelRequest, UpdateLevelRequestMessageId},
		service::level_request_service::LevelRequestService
	},
	send_level::model::request_score::RequestRating,
	serenity::discord::{
		create_thread, extract_command_options, invoke_command_ephemeral, log_action_to_discord,
		log_error_to_discord, send_level_request_message_to_discord
	}
};

pub fn register_request_level<'a>() -> CreateCommand<'a> {
	CreateCommand::new("request-level")
		.description("Request a level to Ryder")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The ID of the level to request."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"request-rating",
				"The amount of Stars/Moons requested."
			)
			.required(true)
			.add_string_choice("Auto, 1 Star/Moon", "One")
			.add_string_choice("Easy, 2 Stars/Moons", "Two")
			.add_string_choice("Normal, 3 Stars/Moons", "Three")
			.add_string_choice("Hard, 4 Stars/Moons", "Four")
			.add_string_choice("Hard, 5 Stars/Moons", "Five")
			.add_string_choice("Harder, 6 Stars/Moons", "Six")
			.add_string_choice("Harder, 7 Stars/Moons", "Seven")
			.add_string_choice("Insane, 8 Stars/Moons", "Eight")
			.add_string_choice("Insane, 9 Stars/Moons", "Nine")
			.add_string_choice("Demon, 10 Stars/Moons", "Ten")
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"video-link",
				"A link to the video showcasing the requested level."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Boolean,
				"request-feedback",
				"Request for reviewers to potentially review your request."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Boolean,
				"notify",
				"Notify when a review has been made or if the level has been sent."
			)
			.required(true)
		)
}

pub async fn run_request_level(ctx: &Context, command: &CommandInteraction) {
	let command_map = extract_command_options(&command);
	let level_request = LevelRequest::new(
		command_map
			.get("level-id")
			.unwrap()
			.as_i64()
			.unwrap()
			.unsigned_abs(),
		command.user.id.get(),
		RequestRating::from_str(
			command_map
				.get("request-rating")
				.unwrap()
				.as_str()
				.unwrap()
		)
		.unwrap(),
		command_map
			.get("video-link")
			.unwrap()
			.as_str()
			.unwrap()
			.to_string(),
		command_map
			.get("request-feedback")
			.unwrap()
			.as_bool()
			.unwrap(),
		command_map
			.get("notify")
			.unwrap()
			.as_bool()
			.unwrap(),
	);

	let service = LevelRequestService::new();
	let content: &str;

	match service.level_request_service(&level_request).await {
		Ok(requested_level) => {
			match send_level_request_message_to_discord(&ctx, &requested_level).await {
				Ok(message_data) => {
					if let Err(create_thread_error) =
						create_thread(&ctx, &command.user, message_data.id.get(), &requested_level).await
					{
						error!("Error creating thread: {}", create_thread_error);
						log_error_to_discord(
							&command.user,
							"creating thread for requested level",
							&create_thread_error,
							Some(&requested_level),
							&ctx
						)
						.await;
					}

					let update_request_message_id = UpdateLevelRequestMessageId {
						level_id: requested_level.level_id,
						discord_message_id: message_data.id.get()
					};
					if let Err(error) = service
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
			content = "Level has been requested successfully!";
			invoke_command_ephemeral(&content, &ctx, &command).await;

			log_action_to_discord(
				&command.user,
				"requested a level",
				Some(&requested_level),
				&ctx
			)
			.await;
		}
		Err(request_level_error) => {
			invoke_command_ephemeral(&request_level_error.to_string(), &ctx, &command).await;
			log_error_to_discord(
				&command.user,
				"requesting a level",
				&request_level_error,
				None,
				&ctx
			)
			.await;
		}
	}
}

pub fn register_edit_level_request<'a>() -> CreateCommand<'a> {
	CreateCommand::new("edit-level-request")
		.description("Edits an existing level request")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the level request to edit."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"request-rating",
				"The amount of Stars/Moons requested."
			)
			.add_string_choice("Auto, 1 Star/Moon", "One")
			.add_string_choice("Easy, 2 Stars/Moons", "Two")
			.add_string_choice("Normal, 3 Stars/Moons", "Three")
			.add_string_choice("Hard, 4 Stars/Moons", "Four")
			.add_string_choice("Hard, 5 Stars/Moons", "Five")
			.add_string_choice("Harder, 6 Stars/Moons", "Six")
			.add_string_choice("Harder, 7 Stars/Moons", "Seven")
			.add_string_choice("Insane, 8 Stars/Moons", "Eight")
			.add_string_choice("Insane, 9 Stars/Moons", "Nine")
			.add_string_choice("Demon, 10 Stars/Moons", "Ten")
		)
		.add_option(CreateCommandOption::new(
			CommandOptionType::String,
			"video-link",
			"A link to the video showcasing the requested level."
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"request-feedback",
			"Request for reviewers to potentially review your request."
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"notify",
			"Notify when a review has been made or if the level has been sent."
		))
}

pub async fn run_edit_level_request(ctx: &Context, command: &CommandInteraction) {
	let content: String;
	let command_map = extract_command_options(&command);
	let update_level_request = UpdateLevelRequest::new(
		command.user.id.get(),
		command_map
			.get("level-id")
			.unwrap()
			.as_i64()
			.unwrap()
			.unsigned_abs(),
		if let Some(request_rating) = command_map.get("request-rating") {
			Some(RequestRating::from_str(request_rating.as_str().unwrap()).unwrap())
		} else {
			None
		},
		if let Some(video_link) = command_map.get("video_link") {
			Some(video_link.as_str().unwrap().to_string())
		} else {
			None
		},
		if let Some(has_requested_feedback) = command_map.get("request-feedback") {
			Some(has_requested_feedback.as_bool().unwrap())
		} else {
			None
		},
		if let Some(notify) = command_map.get("notify") {
			Some(notify.as_bool().unwrap())
		} else {
			None
		}
	);
	let service = LevelRequestService::new();

	match service.update_level_request(update_level_request).await {
		Ok(level_request_data) => {
			if let Err(edit_message_error) =
				send_level_request_message_to_discord(&ctx, &level_request_data).await
			{
				error!(
					"Unable to edit level request message: {}",
					edit_message_error
				);

				content = "Unable to edit level request message.".to_string();
			} else {
				log_action_to_discord(
					&command.user,
					"edited a level request",
					Some(&level_request_data),
					&ctx
				)
				.await;

				content = "Level request has been edited successfully!".to_string();
			}
		}
		Err(error) => {
			content = error.to_string();
			log_error_to_discord(&command.user, "editing a level request", &error, None, &ctx)
				.await;
		}
	}

	invoke_command_ephemeral(&content, &ctx, &command).await;
}

pub fn register_delete_level_request<'a>() -> CreateCommand<'a> {
	CreateCommand::new("delete-level-request")
		.description("Deletes an existing level request")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the level request to delete."
			)
			.required(true)
		)
}

pub async fn run_delete_level_request(ctx: &Context, command: &CommandInteraction) {
	let content: String;
	if !command.user.id.eq(&CLIENT_CONFIG.discord_bot_admin_id) {
		content = "Forbidden".to_string();
		invoke_command_ephemeral(&content, &ctx, &command).await;
		return;
	}

	let command_map = extract_command_options(&command);
	let level_id = command_map
		.get("level-id")
		.unwrap()
		.as_i64()
		.unwrap()
		.unsigned_abs();
	let service = LevelRequestService::new();

	match service.delete_level_request(level_id).await {
		Ok(level_request) => {
			if let Err(delete_message_error) =
				GenericChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
					.delete_message(&ctx.http, MessageId::new(level_request.discord_message_id.unwrap()), None)
					.await
			{
				error!(
					"Unable to delete level request message: {}",
					delete_message_error
				);
			}
			if let Err(delete_message_error) =
				GenericChannelId::new(level_request.discord_message_id.unwrap())
					.delete(&ctx.http, None)
					.await
			{
				error!(
					"Unable to delete level request thread: {:?}",
					delete_message_error
				);
			}
			log_action_to_discord(
				&command.user,
				"deleted a level request",
				Some(&level_request),
				&ctx
			)
			.await;

			content = "Level request has been deleted successfully!".to_string();
		}
		Err(error) => {
			content = error.to_string();
			log_error_to_discord(
				&command.user,
				"deleting a level request",
				&error,
				None,
				&ctx
			)
			.await;
		}
	}

	invoke_command_ephemeral(&content, &ctx, &command).await;
}
