mod config;

mod level_request;
mod level_review;
mod request_manager;
mod requestx_api;
mod reviewer;
mod send_level;
mod serenity;
mod user;

use crate::config::app_config::init_app_config;
use ::serenity::secrets::Token;
use ::serenity::{prelude::GatewayIntents, Client};
use log::error;
use log4rs::config::Deserializers;

#[tokio::main]
async fn main() {
	log4rs::init_file("log4rs.yml", Deserializers::new()).unwrap();
	init_app_config();

	let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
	let mut client = Client::builder(
		Token::from_env("REQUESTX_DISCORD_BOT_TOKEN").unwrap(),
		intents,
	)
	.event_handler(serenity::handler::Handler)
	.await
	.expect("Error creating client");
	if let Err(why) = client.start().await {
		error!("Client error: {why:?}");
	}
}
