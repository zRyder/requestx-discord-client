use std::env;

use async_trait::async_trait;
use serenity::{
	all::{GuildId, Interaction, Ready},
	prelude::{Context, EventHandler}
};

use crate::commands::{request_level, review, reviewer};

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
			.set_commands(
				&ctx.http,
				vec![
					request_level::register(),
					review::register_review(),
					reviewer::register_add_reviewer(),
					reviewer::register_remove_reviewer(),
				]
			)
			.await;
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::Command(command) = interaction {
			println!("Received command interaction: {command:#?}");

			match command.data.name.as_str() {
				"request-level" => request_level::run(&ctx, &command).await,
				"review" => review::post_level_review(&ctx, &command).await,
				"add-reviewer" => reviewer::run_add_reviewer(&ctx, &command).await,
				"remove-reviewer" => reviewer::run_remove_reviewer(&ctx, &command).await,
				_ => println!("Unreachable")
			};
		}
	}
}
