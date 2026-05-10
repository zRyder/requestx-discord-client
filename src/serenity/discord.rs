use chrono::Utc;
use log::error;
use serenity::all::{ButtonStyle, ComponentInteraction, CreateButton, GenericChannelId, Mention, MessageId, ModalInteraction, UserId};
use serenity::{
	all::{
		ChannelId, CommandDataOptionValue, CommandInteraction, Context, CreateEmbed,
		CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
		CreateInteractionResponseMessage, CreateMessage, CreateThread, EditMessage, Message,
		MessageBuilder, User,
	},
	Error as SerenityError,
};
use std::{collections::HashMap, error::Error, fmt::Debug};
use tokio::{sync::mpsc, task};

use crate::request_manager::format_cooldown_duration_string;
use crate::request_manager::model::request_manager::RequestConfig;
use crate::{
	config::client_config::CLIENT_CONFIG, level_request::model::level_request::LevelRequest,
	send_level::model::request_score::LevelLength,
};
use crate::send_level::model::moderator::{SentLevel, SuggestedScore};

pub fn extract_command_options(
	command: &CommandInteraction,
) -> HashMap<&str, &CommandDataOptionValue> {
	command
		.data
		.options
		.iter()
		.map(|data| (data.name.as_str(), &data.value))
		.collect::<HashMap<&str, &CommandDataOptionValue>>()
}

pub async fn log_action_to_discord(
	user: &User,
	operation_message: &str,
	extra_log_ctx: Option<&(dyn Debug + Send + Sync)>,
	ctx: &Context,
) {
	let mut log_message = MessageBuilder::new();
	log_message = log_message.push_bold(format!("{} ", user.name).as_str());
	log_message =
		log_message.push_line(format!("({}) has {}", user.id, operation_message).as_str());
	if let Some(extra_log_ctx) = extra_log_ctx {
		log_message =
			log_message.push_codeblock(format!("{:?}", &extra_log_ctx).as_str(), Some("rust"));
	}

	log_to_discord(ctx.clone(), log_message.build()).await
}

pub async fn log_error_to_discord(
	user: &User,
	operation_message: &str,
	error: &(dyn Error + Send + Sync),
	extra_log_ctx: Option<&(dyn Debug + Send + Sync)>,
	ctx: &Context,
) {
	let mut log_message = MessageBuilder::new();
	log_message = log_message.push_bold(format!("{} ", user.name).as_str());
	log_message = log_message
		.push_line(format!("({}) cause an error when {}", user.id, operation_message).as_str());
	log_message = log_message.push_codeblock(format!("{:?}", error).as_str(), Some("rust"));
	if let Some(extra_log_ctx) = extra_log_ctx {
		log_message =
			log_message.push_codeblock(format!("{:?}", extra_log_ctx).as_str(), Some("rust"));
	};

	log_to_discord(ctx.clone(), log_message.build()).await
}

pub async fn create_thread(
	ctx: &Context,
	user: &User,
	message_id: u64,
	level_request: &LevelRequest,
) -> Result<u64, SerenityError> {
	let thread_name = if let Some(gd_level_info) = &level_request.gd_level_info {
		format!(
			"\"{}\" ({})",
			gd_level_info.level_name, level_request.level_id
		)
	} else {
		format!("{}", level_request.level_id)
	};

	ChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
		.create_thread_from_message(
			&ctx.http,
			MessageId::new(message_id),
			CreateThread::new(thread_name)
				.audit_log_reason(&*format!(
					"Created via command by: {} {}",
					user.name, user.id,
				))
				.invitable(false),
		)
		.await
		.map(|thread_channel| thread_channel.id.get())
}

pub async fn send_level_request_message_to_discord(
	ctx: &Context,
	level_request: &LevelRequest,
) -> serenity::Result<Message> {
	let mut request_message = MessageBuilder::new();
	request_message =
		request_message.push_line(build_level_id_and_name_string(&level_request).as_str());
	request_message =
		request_message.push_line(build_request_rating_string(&level_request).as_str());
	if let Some(request_feedback_string) = build_request_feedback_string(&level_request) {
		request_message = request_message.push_line(request_feedback_string.as_str())
	};
	request_message =
		request_message.push_line(format!("{}", &level_request.youtube_video_link).as_str());

	match &level_request.discord_message_id {
		Some(discord_message_id) => {
			GenericChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
				.edit_message(
					&ctx.http,
					MessageId::new(*discord_message_id),
					EditMessage::new().content(request_message.build()),
				)
				.await
		}
		None => {
			GenericChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
				.send_message(
					&ctx.http,
					CreateMessage::new().content(request_message.build()),
				)
				.await
		}
	}
}

fn build_level_id_and_name_string(level_request: &LevelRequest) -> String {
	let mut level_id_and_name_string = String::new();
	if let Some(gd_level_info) = &level_request.gd_level_info {
		level_id_and_name_string.push_str(&format!(
			"\"{}\" by {}\n",
			gd_level_info.level_name, gd_level_info.level_author
		));
	}
	level_id_and_name_string.push_str(format!("{}", &level_request.level_id).as_str());
	level_id_and_name_string
}

fn build_request_rating_string(level_request: &LevelRequest) -> String {
	let mut request_rating_string = String::new();
	if let Some(gd_level_info) = &level_request.gd_level_info {
		let level_rating_str = &level_request.request_rating.to_string();
		let slice: Vec<&str> = level_rating_str.split(&[' ', '/'][..]).collect();
		let output_str;
		match gd_level_info.level_length {
			LevelLength::Platformer => {
				output_str = format!(
					"{} {} {}",
					slice.get(0).unwrap(),
					slice.get(1).unwrap(),
					slice.get(3).unwrap()
				);
			}
			_ => {
				output_str = format!(
					"{} {} {}",
					slice.get(0).unwrap(),
					slice.get(1).unwrap(),
					slice.get(2).unwrap()
				);
			}
		}
		request_rating_string.push_str(&format!("Requested {}", output_str).as_str());
	} else {
		request_rating_string
			.push_str(&format!("Requested {}", level_request.request_rating).as_str());
	}

	request_rating_string
}

fn build_request_feedback_string(level_request: &LevelRequest) -> Option<String> {
	let mut request_rating_string = String::new();
	if level_request.has_requested_feedback {
		request_rating_string.push_str("Feedback has been requested!");
		Some(request_rating_string)
	} else {
		None
	}
}

pub async fn invoke_command_ephemeral(content: &str, ctx: &Context, command: &CommandInteraction) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = command.create_response(&ctx.http, builder).await {
		error!("Cannot respond to slash command: {err}");
	}
}

pub async fn invoke_modal_ephemeral(
	content: &str,
	ctx: &Context,
	modal_interaction: &ModalInteraction,
) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = modal_interaction.create_response(&ctx.http, builder).await {
		error!("Cannot respond to modal: {err}");
	}
}

pub async fn invoke_component_ephemeral(
	content: &str,
	ctx: &Context,
	component: &ComponentInteraction,
) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = component.create_response(&ctx.http, builder).await {
		error!("Cannot respond to component: {err}");
	}
}

async fn discord_log(mut rx: mpsc::Receiver<(String, Context)>) {
	while let Some(data) = rx.recv().await {
		if let Err(logger_error) = GenericChannelId::new(CLIENT_CONFIG.discord_log_channel_id)
			.say(&data.1.http, &data.0)
			.await
		{
			error!(
				"Unable to log event {} to Discord: {}",
				data.0, logger_error
			);
		}
	}
}

pub async fn log_to_discord(ctx: Context, log_text: String) {
	let (tx, rx) = mpsc::channel::<(String, Context)>(32);
	task::spawn(discord_log(rx));
	tx.send((log_text, ctx)).await.unwrap();
	drop(tx);
}

pub fn get_request_level_embed<'a>(bot_user: &User) -> CreateEmbed<'a> {
	let mut request_level_message_embed = CreateEmbed::new();

	request_level_message_embed =
		request_level_message_embed.footer(CreateEmbedFooter::new("request"));
	request_level_message_embed =
		request_level_message_embed.author(CreateEmbedAuthor::from(bot_user.clone()));
	request_level_message_embed = request_level_message_embed.timestamp(Utc::now());

	request_level_message_embed = request_level_message_embed.field(
		"",
		format!(
			"In order to make a level request, press the \"Request a Level\" button below!\
			\n\n\
			If you would instead like to edit an existing level request, press then \
			\"Edit Existing Level Request\" button. \
			\n\n\
			For more help, check <#{}>",
			1311023368493072514u64
		),
		false,
	);

	request_level_message_embed
}

pub fn get_request_level_config_embed<'a>(
	bot_user: &'a User,
	request_config: &RequestConfig,
) -> CreateEmbed<'a> {
	let mut request_level_message_embed = CreateEmbed::new();

	request_level_message_embed =
		request_level_message_embed.footer(CreateEmbedFooter::new("request-config"));
	request_level_message_embed =
		request_level_message_embed.author(CreateEmbedAuthor::from(bot_user.clone()));
	request_level_message_embed = request_level_message_embed.timestamp(Utc::now());

	let requests_enabled_string = if request_config
		.enable_requests
		.is_some_and(|requests_enabled| requests_enabled) {
		MessageBuilder::new().push_bold("Enabled").build()
	} else {
		MessageBuilder::new().push_bold("Disabled").build()
	};
	let request_cooldown_string = if let Some(request_cooldown) = request_config.duration_in_minutes {
		if let Some(cooldown_string) = format_cooldown_duration_string(request_cooldown) {
			MessageBuilder::new()
				.push_bold(cooldown_string.as_str())
				.build()
		} else {
			MessageBuilder::new().push_bold("No cooldown").build()
		}
	} else {
		MessageBuilder::new().push_bold("Disabled").build()
	};
	let allow_non_user_created_level_requests_string = if request_config
		.allow_non_user_created_levels
		.is_some_and(|allow_non_user_created_levels| allow_non_user_created_levels) {
		MessageBuilder::new().push_bold("CAN").build()
	} else {
		MessageBuilder::new().push_bold("CANNOT").build()
	};
	let allow_platformer_level_requests_string = if request_config
		.allow_platformer_levels
		.is_some_and(|allow_platformer_levels| allow_platformer_levels) {
		MessageBuilder::new().push_bold("CAN").build()
	} else {
		MessageBuilder::new().push_bold("CANNOT").build()
	};
	let gd_request_enabled = if request_config
		.enable_gd_requests
		.is_some_and(|enable_gd_requests| enable_gd_requests)
	{
		MessageBuilder::new().push_bold("WILL").build()
	} else {
		MessageBuilder::new().push_bold("WILL NOT").build()
	};

	request_level_message_embed = request_level_message_embed
		.field(
			"Request Enabled",
			format!("Level requests are currently: {}", requests_enabled_string),
			false,
		)
		.field(
			"Request Cooldown",
			format!(
				"The request cooldown is currently: {}",
				request_cooldown_string
			),
			false,
		)
		.field(
			"User Created Level Requests",
			format!(
				"You {} request levels that you have not created",
				allow_non_user_created_level_requests_string
			),
			false,
		)
		.field(
			"Platformer Level Requests",
			format!(
				"You {} request platformer levels",
				allow_platformer_level_requests_string
			),
			false,
		)
		.field(
			"Geometry Dash Integration",
			format!(
				"Your level requests {} automatically populate with in-game info",
				gd_request_enabled
			),
			false,
		);

	request_level_message_embed
}

pub fn get_init_gd_account_link_embed<'a>(bot_user: &User) -> CreateEmbed<'a> {
	let mut init_message_embed = CreateEmbed::new();

	init_message_embed = init_message_embed.footer(CreateEmbedFooter::new("init"));
	init_message_embed = init_message_embed.author(CreateEmbedAuthor::from(bot_user.clone()));
	init_message_embed = init_message_embed.timestamp(Utc::now());
	init_message_embed = init_message_embed.field(
		"",
		format!(
			"In order to make level request while Ryder is taking user created \
			level requests only, you must link your GD account. In order to link your GD account \
            follow the instructions below:\n\
            - Press the \"Link GD Account\" button below\n\
            - Enter your GD username when prompted and press submit\n\
            - Copy the token that will be sent in this channel\n\
            - Make a profile post containing that token\n\
            - Press the \"Verify GD Account Link\" button in the below embed\
            \n\n\
			Once you have posted the token from the above step \
			click the \"Verify GD Account Link\" button below.\
			\n\n\
            If you need assistance, send a DM to <@{}>.",
			CLIENT_CONFIG.discord_bot_admin_id
		),
		false,
	);

	init_message_embed
}

pub fn get_verify_level_request_buttons<'a>() -> Vec<CreateButton<'a>> {
	vec![
		CreateButton::new("verify-request-level-yes-button")
			.label("Yes")
			.style(ButtonStyle::Success),
		CreateButton::new("verify-request-level-no-button")
			.label("No")
			.style(ButtonStyle::Danger),
	]
}

pub fn format_public_discord_message(sent_level: &SentLevel) -> MessageBuilder {
	let mut send_level_message = MessageBuilder::new();

	send_level_message = send_level_message.push(build_level_name_string(&sent_level).as_str());
	send_level_message = send_level_message.push(build_sent_for_string(&sent_level).as_str());
	send_level_message = send_level_message.push(build_notify_string(&sent_level).as_str());

	send_level_message
}

fn build_level_name_string(sent_level: &SentLevel) -> String {
	let mut level_name_string = String::new();
	if let Some(gd_level_info) = &sent_level.level_request.gd_level_info {
		level_name_string.push_str(&format!("\"{}\" ", &gd_level_info.level_name));
	} else {
		level_name_string.push_str("The level ");
	}

	level_name_string
}

fn build_sent_for_string(sent_level: &SentLevel) -> String {
	let mut level_sent_for_string = MessageBuilder::new();
	if sent_level.moderator_data.suggested_score == SuggestedScore::NoRate {
		level_sent_for_string = level_sent_for_string.push_bold("has not ");
		level_sent_for_string = level_sent_for_string.push("been sent...");
	} else if sent_level.moderator_data.suggested_score == SuggestedScore::Rated {
		level_sent_for_string = level_sent_for_string.push_bold("has already ");
		level_sent_for_string = level_sent_for_string.push("been rated.");
	} else {
		level_sent_for_string = level_sent_for_string.push_bold("has ");
		level_sent_for_string = level_sent_for_string.push("been sent for ");
		level_sent_for_string = level_sent_for_string.push(build_sent_for_with_rating_string(&sent_level).as_str());
	}

	level_sent_for_string.build()
}

fn build_sent_for_with_rating_string(sent_level: &SentLevel) -> String {
	let mut level_sent_for_with_rating_string = MessageBuilder::new();
	level_sent_for_with_rating_string = level_sent_for_with_rating_string.push_bold(
		format!(
			"{}, {} {}",
			serde_json::to_string(&sent_level.moderator_data.suggested_rating)
				.unwrap()
				.replace("\"", ""),
			serde_json::to_string(&sent_level.moderator_data.suggested_score)
				.unwrap()
				.replace("\"", ""),
			if let Some(gd_level_info) = &sent_level.level_request.gd_level_info {
				if gd_level_info.level_length == LevelLength::Platformer {
					"Moons!"
				} else {
					"Stars!"
				}
			} else {
				"Stars/Moons!"
			}
		)
			.as_str(),
	);

	level_sent_for_with_rating_string.build()
}

fn build_notify_string(sent_level: &SentLevel) -> String {
	let mut notify_string = MessageBuilder::new();
	if sent_level.level_request.notify {
		notify_string = notify_string.push(
			format!(
				"\n{}",
				Mention::User(UserId::new(sent_level.level_request.discord_user_id))
			)
				.as_str(),
		);
	}

	notify_string.build()
}
