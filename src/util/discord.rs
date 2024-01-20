use log::error;
use serenity::{
	all::{
		ChannelId, CommandInteraction, Context, CreateInteractionResponse,
		CreateInteractionResponseMessage, CreateThread
	},
	Error
};
use tokio::sync::mpsc;
use tokio::task;
use crate::config::client_config::CLIENT_CONFIG;

use crate::model::requestx_api::level_request_data::LevelRequestData;

pub async fn create_thread(
	ctx: &Context,
	command: &CommandInteraction,
	message_id: u64,
	level: &LevelRequestData
) -> Result<u64, Error> {
	match ChannelId::new(1193493680594616411)
		.create_thread_from_message(
			&ctx.http,
			message_id,
			CreateThread::new(format!("\"{}\" ({})", level.level_name, level.level_id))
				.audit_log_reason(&*format!(
					"Created via {} command by: {} {}",
					command.data.name, command.user.name, command.user.id,
				))
				.invitable(false)
		)
		.await
	{
		Ok(thread_channel_id) => Ok(thread_channel_id.id.get()),
		Err(error) => Err(error)
	}
}

pub async fn invoke_ephermal(content: &str, ctx: &Context, command: &CommandInteraction) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = command.create_response(&ctx.http, builder).await {
		println!("Cannot respond to slash command: {err}");
	}
}

async fn discord_log(mut rx: mpsc::Receiver<(String, Context)>) {
	while let Some(data) = rx.recv().await {
		if let Err(logger_error) = ChannelId::new(CLIENT_CONFIG.discord_log_channel_id)
			.say(&data.1.http, &data.0)
			.await {
			error!("Unable to log event {} to Discord: {}", data.0, logger_error);
		}
	}
}

pub async fn log_to_discord(log_text: String, ctx: Context) {
	let (tx, rx) = mpsc::channel::<(String, Context)>(32);
	task::spawn(discord_log(rx));
	tx.send((log_text, ctx)).await.unwrap();
	drop(tx);
}
