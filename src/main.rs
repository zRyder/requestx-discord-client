mod commands;
mod config;
mod model;

mod serenity;
mod service;

use std::process;

use ::serenity::{prelude::GatewayIntents, Client};
use log::error;

use crate::config::common_config::{init_app_config, APP_CONFIG};

#[tokio::main]
async fn main() {
	log4rs::init_file("log4rs.yml", Default::default()).unwrap();
	if let Err(error) = init_app_config() {
		error!("Error loading app config: {}", error);
		process::exit(1)
	} else {
		let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
		let mut client = Client::builder(&APP_CONFIG.client_config.discord_bot_token, intents)
			.event_handler(serenity::handler::Handler)
			.await
			.expect("Error creating client");
		if let Err(why) = client.start().await {
			error!("Client error: {why:?}");
		}
	}
}
