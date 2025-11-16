use std::str::FromStr;

use log::error;
use serenity::all::{
	ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
	MessageBuilder
};

use crate::{
	config::client_config::CLIENT_CONFIG,
	model::{
		moderator::{SendLevelRequest, SentLevel, SuggestedRating, SuggestedScore},
		request_score::LevelLength
	},
	serenity::discord::{
		extract_command_options, invoke_ephemeral, log_action_to_discord, log_error_to_discord
	},
	service::moderator_service::ModeratorService
};

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
			.add_string_choice("Already Rated", "Rated")
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
	let content: String;
	if !command.user.id.eq(&CLIENT_CONFIG.discord_bot_admin_id) {
		content = "Forbidden".to_string();
		invoke_ephemeral(&content, &ctx, &command).await;
		return;
	}

	let command_map = extract_command_options(&command);

	let level_id = command_map
		.get(&"level-id".to_string())
		.unwrap()
		.as_i64()
		.unwrap()
		.unsigned_abs();
	let suggested_score = SuggestedScore::from_str(
		command_map
			.get(&"suggested-score".to_string())
			.unwrap()
			.as_str()
			.unwrap()
	)
	.unwrap();
	let suggested_rating = SuggestedRating::from_str(
		command_map
			.get(&"suggested-rating".to_string())
			.unwrap()
			.as_str()
			.unwrap()
	)
	.unwrap();
	let send_level_request = SendLevelRequest::new(level_id, suggested_score, suggested_rating);
	let service = ModeratorService::new();

	match service.send_level(send_level_request).await {
		Ok(sent_level) => {
			let mut send_level_message =
				format_public_discord_message(&suggested_score, &suggested_rating, &sent_level);

			match ChannelId::new(sent_level.level_request.discord_message_id.unwrap())
				.say(&ctx.http, &send_level_message.build())
				.await
			{
				Ok(_msg) => {
					content = "Level has been sent!".to_string();
					log_action_to_discord(
						&command.user,
						"sent a level to RobTop",
						Some(&sent_level),
						&ctx
					)
					.await;
				}
				Err(error) => {
					error!("{}", error);
					content = "Error sending message.".to_string();
				}
			}
		}
		Err(send_level_error) => {
			content = send_level_error.to_string();
			log_error_to_discord(
				&command.user,
				"sending level to RobTop",
				&send_level_error,
				None,
				&ctx
			)
			.await;
		}
	}

	invoke_ephemeral(&content, &ctx, &command).await;
}

fn format_public_discord_message(
	suggested_score: &SuggestedScore,
	suggested_rating: &SuggestedRating,
	sent_level: &SentLevel
) -> MessageBuilder {
	let mut send_level_message = MessageBuilder::new();

	if let Some(ref level_name) = sent_level.level_request.level_name {
		send_level_message.push(format!("\"{}\" ", level_name));
	} else {
		send_level_message.push("The level ");
	}
	if sent_level.moderator_data.suggested_score == SuggestedScore::NoRate {
		send_level_message.push_bold("has not ");
		send_level_message.push("been sent...");
	} else if sent_level.moderator_data.suggested_score == SuggestedScore::Rated {
		send_level_message.push_bold("has already ");
		send_level_message.push("been rated.");
	} else {
		send_level_message.push_bold("has ");
		send_level_message.push("been sent for ");
		send_level_message.push_bold(format!(
			"{}, {} {}",
			serde_json::to_string(&suggested_rating)
				.unwrap()
				.replace("\"", ""),
			serde_json::to_string(&suggested_score)
				.unwrap()
				.replace("\"", ""),
			if let Some(level_length) = sent_level.level_request.level_length {
				if level_length == LevelLength::Platformer {
					"Moons!"
				} else {
					"Stars!"
				}
			} else {
				"Stars/Moons!"
			}
		));
	}

	send_level_message
}
