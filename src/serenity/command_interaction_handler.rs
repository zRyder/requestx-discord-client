use std::env;

use async_trait::async_trait;
use serenity::{
	all::{GuildId, Interaction, Ready},
	builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
	prelude::{Context, EventHandler}
};

use crate::commands::request_level;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);

		let guild_id = GuildId::new(
			env::var("GUILD_ID")
				.expect("Expected GUILD_ID in environment")
				.parse()
				.expect("GUILD_ID must be an integer")
		);

		let commands = guild_id
			.set_commands(&ctx.http, vec![request_level::register()])
			.await;

		println!("I now have the following guild slash commands: {commands:#?}");
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::Command(command) = interaction {
			println!("Received command interaction: {command:#?}");

			let content = match command.data.name.as_str() {
				"request-level" => Some(request_level::run(&command).await),
				_ => Some("not implemented :(".to_string())
			};

			if let Some(content) = content {
				let data = CreateInteractionResponseMessage::new()
					.ephemeral(true)
					.content(content);
				let builder = CreateInteractionResponse::Message(data);
				if let Err(why) = command.create_response(&ctx.http, builder).await {
					println!("Cannot respond to slash command: {why}");
				}
			}
		}
	}
}
