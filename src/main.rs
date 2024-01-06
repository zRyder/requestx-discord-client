mod commands;
mod config;
mod model;

mod service;

use std::process;

use poise::serenity_prelude as serenity;
use toml::de::Error;

use crate::config::common_config::init_app_config;

struct Data {}

#[tokio::main]
async fn main() {
	let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
	if let Err(error) = init_app_config() {
		eprintln!("Error loading app config: {}", error);
		process::exit(1)
	} else {
		let intents = serenity::GatewayIntents::non_privileged();

		let framework = poise::Framework::builder()
			.options(poise::FrameworkOptions {
				commands: vec![commands::request_level::request_level()],
				..Default::default()
			})
			.setup(|ctx, _ready, framework| {
				Box::pin(async move {
					poise::builtins::register_globally(ctx, &framework.options().commands).await?;
					Ok(Data {})
				})
			})
			.build();

		let client = serenity::ClientBuilder::new(token, intents)
			.framework(framework)
			.await;
		println!("Running");
		client.unwrap().start().await.unwrap();
	}
}
