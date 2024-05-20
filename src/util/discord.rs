use log::error;
use serenity::{
	all::{
		ChannelId, CommandInteraction, Context, CreateInteractionResponse,
		CreateInteractionResponseMessage, CreateMessage, CreateThread, EditMessage, Message,
		MessageBuilder
	},
	Error
};
use tokio::{sync::mpsc, task};

use crate::{
	config::client_config::CLIENT_CONFIG, model::requestx_api::level_request_data::LevelRequestData
};

pub async fn create_thread(
	ctx: &Context,
	command: &CommandInteraction,
	message_id: u64,
	level_request_data: &LevelRequestData
) -> Result<u64, Error> {
	let create_thread_result = if let Some(level_name) = &level_request_data.level_name {
		ChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
			.create_thread_from_message(
				&ctx.http,
				message_id,
				CreateThread::new(format!(
					"\"{}\" ({})",
					level_name, level_request_data.level_id
				))
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
				CreateThread::new(format!("{}", level_request_data.level_id))
					.audit_log_reason(&*format!(
						"Created via {} command by: {} {}",
						command.data.name, command.user.name, command.user.id,
					))
					.invitable(false)
			)
			.await
	};

	match create_thread_result {
		Ok(thread_channel_id) => Ok(thread_channel_id.id.get()),
		Err(error) => Err(error)
	}
}

pub async fn send_level_request_message_to_discord(
	ctx: &Context,
	level_request_data: &LevelRequestData
) -> serenity::Result<Message> {
	let mut request_message = MessageBuilder::new();
	if let (Some(level_name), Some(level_creator_name)) = (
		&level_request_data.level_name,
		&level_request_data.level_author
	) {
		request_message.push_line(format!("\"{}\" by {}", level_name, level_creator_name));
	}
	request_message
		.push_line(format!("{}", &level_request_data.level_id))
		.push_line(format!("Requested {}", &level_request_data.request_score));
	if level_request_data.has_requested_feedback {
		request_message.push_line("Feedback has been requested!");
	}
	request_message.push_line(format!("{}", &level_request_data.youtube_video_link));

	match &level_request_data.discord_message_id {
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

pub async fn invoke_ephermal(content: &str, ctx: &Context, command: &CommandInteraction) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = command.create_response(&ctx.http, builder).await {
		error!("Cannot respond to slash command: {err}");
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
