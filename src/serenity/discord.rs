use std::{collections::HashMap, error::Error, fmt::Debug};

use chrono::Utc;
use log::error;
use serenity::{
	all::{
		ChannelId, CommandDataOptionValue, CommandInteraction, Context, CreateEmbed,
		CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
		CreateInteractionResponseMessage, CreateMessage, CreateThread, EditMessage, Message,
		MessageBuilder, User
	},
	Error as SerenityError
};
use serenity::all::{ComponentInteraction, ModalInteraction};
use tokio::{sync::mpsc, task};

use crate::{
	config::client_config::CLIENT_CONFIG, level_request::model::level_request::LevelRequest,
	send_level::model::request_score::LevelLength
};

pub fn extract_command_options(
	command: &CommandInteraction
) -> HashMap<&String, &CommandDataOptionValue> {
	command
		.data
		.options
		.iter()
		.map(|data| (&data.name, &data.value))
		.collect::<HashMap<&String, &CommandDataOptionValue>>()
}

pub async fn log_action_to_discord(
	user: &User,
	operation_message: &str,
	extra_log_ctx: Option<&(dyn Debug + Send + Sync)>,
	ctx: &Context
) {
	let mut log_message = MessageBuilder::new();
	log_message.push_bold(format!("{} ", user.name));
	log_message.push_line(format!("({}) has {}", user.id, operation_message));
	if let Some(extra_log_ctx) = extra_log_ctx {
		log_message.push_codeblock(format!("{:?}", &extra_log_ctx), Some("rust"));
	}

	log_to_discord(ctx.clone(), log_message.build()).await
}

pub async fn log_error_to_discord(
	user: &User,
	operation_message: &str,
	error: &(dyn Error + Send + Sync),
	extra_log_ctx: Option<&(dyn Debug + Send + Sync)>,
	ctx: &Context
) {
	let mut log_message = MessageBuilder::new();
	log_message.push_bold(format!("{} ", user.name));
	log_message.push_line(format!(
		"({}) cause an error when {}",
		user.id, operation_message
	));
	log_message.push_codeblock(format!("{:?}", error), Some("rust"));
	if let Some(extra_log_ctx) = extra_log_ctx {
		log_message.push_codeblock(format!("{:?}", extra_log_ctx), Some("rust"));
	};

	log_to_discord(ctx.clone(), log_message.build()).await
}

pub async fn create_thread(
	ctx: &Context,
	command: &CommandInteraction,
	message_id: u64,
	level_request: &LevelRequest
) -> Result<u64, SerenityError> {
	let create_thread_result = if let Some(level_name) = &level_request.level_name {
		ChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
			.create_thread_from_message(
				&ctx.http,
				message_id,
				CreateThread::new(format!("\"{}\" ({})", level_name, level_request.level_id))
					.audit_log_reason(&*format!(
						"Created via {} command by: {} {}",
						command.data.name, command.user.name, command.user.id,
					))
					.invitable(false)
			)
			.await
	} else {
		ChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
			.create_thread_from_message(
				&ctx.http,
				message_id,
				CreateThread::new(format!("{}", level_request.level_id))
					.audit_log_reason(&*format!(
						"Created via {} command by: {} {}",
						command.data.name, command.user.name, command.user.id,
					))
					.invitable(false)
			)
			.await
	};

	create_thread_result.map(|thread_channel| thread_channel.id.get())
}

pub async fn send_level_request_message_to_discord(
	ctx: &Context,
	level_request: &LevelRequest
) -> serenity::Result<Message> {
	let mut request_message = MessageBuilder::new();
	if let (Some(level_name), Some(level_creator_name)) =
		(&level_request.level_name, &level_request.level_author)
	{
		request_message.push_line(format!("\"{}\" by {}", level_name, level_creator_name));
	}
	request_message.push_line(format!("{}", &level_request.level_id));
	if let Some(level_length) = level_request.level_length {
		let level_rating_str = level_request.request_rating.to_string();
		let slice: Vec<&str> = level_rating_str.split(&[' ', '/'][..]).collect();
		let output_str;
		match level_length {
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
		request_message.push_line(format!("Requested {}", output_str));
	} else {
		request_message.push_line(format!("Requested {}", level_request.request_rating));
	}
	if level_request.has_requested_feedback {
		request_message.push_line("Feedback has been requested!");
	}
	request_message.push_line(format!("{}", &level_request.youtube_video_link));

	match &level_request.discord_message_id {
		Some(discord_message_id) => {
			ChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
				.edit_message(
					&ctx.http,
					*discord_message_id,
					EditMessage::new().content(request_message.build())
				)
				.await
		}
		None => {
			ChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
				.send_message(
					&ctx.http,
					CreateMessage::new().content(request_message.build())
				)
				.await
		}
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

pub async fn invoke_modal_ephemeral(content: &str, ctx: &Context, modal_interaction: &ModalInteraction) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = modal_interaction.create_response(&ctx.http, builder).await {
		error!("Cannot respond to modal: {err}");
	}
}

pub async fn invoke_component_ephemeral(content: &str, ctx: &Context, component: &ComponentInteraction) {
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
		if let Err(logger_error) = ChannelId::new(CLIENT_CONFIG.discord_log_channel_id)
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

pub fn get_init_gd_account_link_embed(bot_user: &User) -> CreateEmbed {
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
            - Press the \"Verify GD Account Link\" button in the below embed\n\
            \n\n\
            If you need assistance, send a DM to <@{}>.",
			CLIENT_CONFIG.discord_bot_admin_id
		),
		false,
	);

	init_message_embed
}

pub fn get_verify_gd_account_link_embed(bot_user: &User) -> CreateEmbed {
	let mut init_message_embed = CreateEmbed::new();

	init_message_embed = init_message_embed.footer(CreateEmbedFooter::new("verify"));
	init_message_embed = init_message_embed.author(CreateEmbedAuthor::from(bot_user.clone()));
	init_message_embed = init_message_embed.timestamp(Utc::now());
	init_message_embed = init_message_embed.field(
		"",
		"Once you have posted the token from the above step \
		click the \"Verify GD Account Link\" button below.",
		false
	);

	init_message_embed
}
