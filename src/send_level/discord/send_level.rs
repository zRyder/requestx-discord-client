use std::str::FromStr;

use log::error;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GenericChannelId, Mention, MessageBuilder, UserId};

use crate::{
	config::client_config::CLIENT_CONFIG,
	send_level::{
		model::{
			moderator::{SendLevelRequest, SentLevel, SuggestedRating, SuggestedScore},
			request_score::LevelLength,
		},
		service::send_level_service::ModeratorService,
	},
	serenity::discord::{
		extract_command_options, invoke_command_ephemeral, log_action_to_discord,
		log_error_to_discord,
	},
};

pub fn register_send_level<'a>() -> CreateCommand<'a> {
	CreateCommand::new("send-level")
		.description("Concludes a level request by either sending the level or not")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the request to send",
			)
			.required(true),
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"suggested-score",
				"The suggested amount of Stars/Moons this level should reward",
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
			.add_string_choice("Demon, 10 Stars/Moons", "Ten"),
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"suggested-rating",
				"The suggested Feature score this level should have",
			)
			.required(true)
			.add_string_choice("Rate", "Rate")
			.add_string_choice("Feature", "Feature")
			.add_string_choice("Epic", "Epic")
			.add_string_choice("Legendary", "Legendary")
			.add_string_choice("Mythic", "Mythic"),
		)
}

pub async fn run_send_level(ctx: &Context, command: &CommandInteraction) {
	let content: String;
	if !command.user.id.eq(&CLIENT_CONFIG.discord_bot_admin_id) {
		content = "Forbidden".to_string();
		invoke_command_ephemeral(&content, &ctx, &command).await;
		return;
	}

	let command_map = extract_command_options(&command);

	let level_id = command_map
		.get("level-id")
		.unwrap()
		.as_i64()
		.unwrap()
		.unsigned_abs();
	let suggested_score = SuggestedScore::from_str(
		command_map
			.get("suggested-score")
			.unwrap()
			.as_str()
			.unwrap(),
	)
	.unwrap();
	let suggested_rating = SuggestedRating::from_str(
		command_map
			.get("suggested-rating")
			.unwrap()
			.as_str()
			.unwrap(),
	)
	.unwrap();
	let send_level_request = SendLevelRequest::new(level_id, suggested_score, suggested_rating);
	let service = ModeratorService::new();

	match service.send_level(send_level_request).await {
		Ok(sent_level) => {
			let send_level_message = format_public_discord_message(&sent_level);

			match GenericChannelId::new(sent_level.level_request.discord_message_id.unwrap())
				.say(&ctx.http, &send_level_message.build())
				.await {
				Ok(_msg) => {
					content = "Level has been sent!".to_string();
					log_action_to_discord(
						&command.user,
						"sent a level to RobTop",
						Some(&sent_level),
						&ctx,
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
				&ctx,
			)
			.await;
		}
	}

	invoke_command_ephemeral(&content, &ctx, &command).await;
}

fn format_public_discord_message(sent_level: &SentLevel) -> MessageBuilder {
	let mut send_level_message = MessageBuilder::new();

	send_level_message = send_level_message.push(build_level_name_string(&sent_level).as_str());
	send_level_message = send_level_message.push(build_sent_for_string(&sent_level).as_str());
	send_level_message = send_level_message.push(build_notify_string(&sent_level).as_str());

	send_level_message
}

fn build_level_name_string(sent_level: &SentLevel) -> String {
	let mut level_name_string = String::new();
	if let Some(gd_level_info) = &sent_level.level_request.gd_level_info {
		level_name_string.push_str(&format!("\"{}\" ", &gd_level_info.level_name));
	} else {
		level_name_string.push_str("The level ");
	}

	level_name_string
}

fn build_sent_for_string(sent_level: &SentLevel) -> String {
	let mut level_sent_for_string = MessageBuilder::new();
	if sent_level.moderator_data.suggested_score == SuggestedScore::NoRate {
		level_sent_for_string = level_sent_for_string.push_bold("has not ");
		level_sent_for_string = level_sent_for_string.push("been sent...");
	} else if sent_level.moderator_data.suggested_score == SuggestedScore::Rated {
		level_sent_for_string = level_sent_for_string.push_bold("has already ");
		level_sent_for_string = level_sent_for_string.push("been rated.");
	} else {
		level_sent_for_string = level_sent_for_string.push_bold("has ");
		level_sent_for_string = level_sent_for_string.push("been sent for ");
		level_sent_for_string =
			level_sent_for_string.push(build_sent_for_with_rating_string(&sent_level).as_str())
	}

	level_sent_for_string.build()
}

fn build_sent_for_with_rating_string(sent_level: &SentLevel) -> String {
	let mut level_sent_for_with_rating_string = MessageBuilder::new();
	level_sent_for_with_rating_string = level_sent_for_with_rating_string.push_bold(
		format!(
			"{}, {} {}",
			serde_json::to_string(&sent_level.moderator_data.suggested_rating)
				.unwrap()
				.replace("\"", ""),
			serde_json::to_string(&sent_level.moderator_data.suggested_score)
				.unwrap()
				.replace("\"", ""),
			if let Some(gd_level_info) = &sent_level.level_request.gd_level_info {
				if gd_level_info.level_length == LevelLength::Platformer {
					"Moons!"
				} else {
					"Stars!"
				}
			} else {
				"Stars/Moons!"
			}
		)
		.as_str(),
	);

	level_sent_for_with_rating_string.build()
}

fn build_notify_string(sent_level: &SentLevel) -> String {
	let mut notify_string = MessageBuilder::new();
	if sent_level.level_request.notify {
		notify_string = notify_string.push(
			format!(
				"\n{}",
				Mention::User(UserId::new(sent_level.level_request.discord_user_id))
			).as_str()
		);
	}

	notify_string.build()
}
