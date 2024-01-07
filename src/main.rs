mod commands;
mod config;
mod model;

mod serenity;
mod service;

use std::process;

use ::serenity::{prelude::GatewayIntents, Client};

use crate::config::common_config::init_app_config;

#[tokio::main]
async fn main() {
	let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
	if let Err(error) = init_app_config() {
		eprintln!("Error loading app config: {}", error);
		process::exit(1)
	} else {
		let mut client = Client::builder(token, GatewayIntents::empty())
			.event_handler(serenity::command_interaction_handler::Handler)
			.await
			.expect("Error creating client");

		// Finally, start a single shard, and start listening to events.
		//
		// Shards will automatically attempt to reconnect, and will perform exponential
		// backoff until it reconnects.
		if let Err(why) = client.start().await {
			println!("Client error: {why:?}");
		}
	}
}
