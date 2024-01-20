use std::str::FromStr;

use log::error;
use serenity::all::{
	ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
	Mentionable, MessageBuilder, UserId
};

use crate::{
	model::{
		moderator::{Moderator, SuggestedRating, SuggestedScore},
		request_score::LevelLength,
		requestx_api::moderator_data::ModeratorError
	},
	service::moderator_service::ModeratorService,
	util::discord::invoke_ephermal
};
use crate::util::discord::log_to_discord;

pub fn register_send_level() -> CreateCommand {
	CreateCommand::new("send-level")
		.description("Concludes a level request by either sending the level or not")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the request to send"
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"suggested-score",
				"The suggested amount of Stars/Moons this level should reward"
			)
			.required(true)
			.add_string_choice("No Send", "NoRate")
			.add_string_choice("Auto, 1 Star/Moon", "One")
			.add_string_choice("Easy, 2 Stars/Moons", "Two")
			.add_string_choice("Normal, 3 Stars/Moons", "Three")
			.add_string_choice("Hard, 4 Stars/Moons", "Four")
			.add_string_choice("Hard, 5 Stars/Moons", "Five")
			.add_string_choice("Harder, 6 Stars/Moons", "Six")
			.add_string_choice("Harder, 7 Stars/Moons", "Seven")
			.add_string_choice("Insane, 8 Stars/Moons", "Eight")
			.add_string_choice("Insane, 9 Stars/Moons", "Nine")
			.add_string_choice("Demon, 10 Stars/Moons", "Ten")
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"suggested-rating",
				"The suggested Feature score this level should have"
			)
			.required(true)
			.add_string_choice("Rate", "Rate")
			.add_string_choice("Feature", "Feature")
			.add_string_choice("Epic", "Epic")
			.add_string_choice("Legendary", "Legendary")
			.add_string_choice("Mythic", "Mythic")
		)
}

pub async fn run_send_level(ctx: &Context, command: &CommandInteraction) {
	let level_id = command
		.data
		.options
		.get(0)
		.unwrap()
		.value
		.as_i64()
		.unwrap()
		.unsigned_abs();
	let suggested_score =
		SuggestedScore::from_str(command.data.options.get(1).unwrap().value.as_str().unwrap())
			.unwrap();
	let suggested_rating =
		SuggestedRating::from_str(command.data.options.get(2).unwrap().value.as_str().unwrap())
			.unwrap();
	let send_level_request = Moderator {
		level_id,
		suggested_score,
		suggested_rating
	};
	let service = ModeratorService::new();
	let content;

	match service.send_level(&ctx, &command, send_level_request).await {
		Ok(level_request_data) => {
			let mut send_level_message = MessageBuilder::new();
			send_level_message.push(format!(
				"\"{}\" ({}) ",
				level_request_data.level_name, level_request_data.level_id
			));
			if level_request_data.level_length == LevelLength::Platformer {
				if send_level_request.suggested_score == SuggestedScore::NoRate {
					send_level_message.push_bold("has not ");
					send_level_message.push("been sent...");
				} else {
					send_level_message.push_bold("has ");
					send_level_message.push("been sent for ");
					send_level_message.push_bold(format!(
						"{}, {} Moons!",
						serde_json::to_string(&suggested_rating)
							.unwrap()
							.replace("\"", ""),
						serde_json::to_string(&suggested_score)
							.unwrap()
							.replace("\"", "")
					));
				}
			} else {
				if send_level_request.suggested_score == SuggestedScore::NoRate {
					send_level_message.push_bold("has not ");
					send_level_message.push("been sent...");
				} else {
					send_level_message.push_bold("has ");
					send_level_message.push("been sent for ");
					send_level_message.push_bold(format!(
						"{}, {} Stars!",
						serde_json::to_string(&suggested_rating)
							.unwrap()
							.replace("\"", ""),
						serde_json::to_string(&suggested_score)
							.unwrap()
							.replace("\"", "")
					));
				}
			}

			if level_request_data.notify {
				send_level_message.push_line("");
				send_level_message.push_line(format!(
					"{}",
					UserId::new(level_request_data.discord_id).mention()
				));
			}

			match ChannelId::new(level_request_data.discord_thread_id.unwrap())
				.say(&ctx.http, &send_level_message.build())
				.await
			{
				Ok(_msg) => {
					content = "Level has been sent!".to_string();
					invoke_ephermal(&content, &ctx, &command).await;

					{
						let mut log_message = MessageBuilder::new();
						log_message.push_line("Level has been sent to RobTop".to_string());
						log_message.push_codeblock(format!("{:?}", level_request_data), Some("rust"));
						log_message.push_codeblock(format!("{:?}", send_level_request), Some("rust"));
						log_to_discord(
							log_message.build(),
							ctx.clone(),
						).await
					}
				}
				Err(error) => {
					error!("{}", error);
					content = "Error sending message.".to_string();
					invoke_ephermal(&content, &ctx, &command).await;
				}
			}
		}
		Err(send_level_error) => {
			match send_level_error {
				ModeratorError::LevelRequestDoesNotExist => {
					content = "Level request does not exist.".to_string();
					invoke_ephermal(&content, &ctx, &command).await;
				}
				ModeratorError::RequestXApiError => {
					content = "There was an error making the request".to_string();
					invoke_ephermal(&content, &ctx, &command).await;
				}
				ModeratorError::SerializeError => {
					content = "Unable to serialize request".to_string();
					invoke_ephermal(&content, &ctx, &command).await;
				}
				ModeratorError::RequestError => {
					content = "There was an error making the request".to_string();
					invoke_ephermal(&content, &ctx, &command).await;
				}
			}

			{
				let mut log_message = MessageBuilder::new();
				log_message.push_line("Unable to send level to RobTop".to_string());
				log_message.push_codeblock(format!("{:?}", send_level_request), Some("rust"));
				log_to_discord(
					log_message.build(),
					ctx.clone(),
				).await
			}
		}
	}
}
