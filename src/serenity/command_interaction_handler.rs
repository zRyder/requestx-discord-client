use async_trait::async_trait;
use log::{debug, error, info};
use serenity::{
	all::{GuildId, Interaction, Message, MessageType, Ready},
	prelude::{Context, EventHandler}
};

use crate::{
	commands::{request_level, request_manager, review, reviewer, send_level},
	config::client_config::CLIENT_CONFIG
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, message: Message) {
		if message.kind.eq(&MessageType::ThreadCreated)
			|| message
				.channel_id
				.eq(&CLIENT_CONFIG.discord_public_channel_id)
		{
			if !message
				.author
				.has_role(
					&ctx.http,
					CLIENT_CONFIG.discord_guild_id,
					CLIENT_CONFIG.discord_maintenance_role_id
				)
				.await
				.unwrap()
			{
				if let Err(message_delete_error) = message.delete(&ctx.http).await {
					error!("Unable to delete message: {}", message_delete_error);
				}
			}
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);

		let guild_id = GuildId::new(CLIENT_CONFIG.discord_guild_id);

		guild_id
			.set_commands(
				&ctx.http,
				vec![
					request_level::register_request_level(),
					request_level::register_edit_level_request(),
					request_level::register_delete_level_request(),
					review::register_review(),
					reviewer::register_add_reviewer(),
					reviewer::register_remove_reviewer(),
					send_level::register_send_level(),
					request_manager::register_request_manager(),
				]
			)
			.await
			.expect("Unable to set commands");
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::Command(command) = interaction {
			debug!("Received command interaction: {command:#?}");

			match command.data.name.as_str() {
				"request-level" => request_level::run_request_level(&ctx, &command).await,
				"edit-level-request" => request_level::run_edit_level_request(&ctx, &command).await,
				"delete-level-request" => {
					request_level::run_delete_level_request(&ctx, &command).await
				}
				"review" => review::post_level_review(&ctx, &command).await,
				"add-reviewer" => reviewer::run_add_reviewer(&ctx, &command).await,
				"remove-reviewer" => reviewer::run_remove_reviewer(&ctx, &command).await,
				"send-level" => send_level::run_send_level(&ctx, &command).await,
				"request-manager" => request_manager::run_request_manager(&ctx, &command).await,
				_ => println!("Unreachable")
			};
		}
	}
}
