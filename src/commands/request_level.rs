use std::fmt::format;
use std::str::FromStr;

use serenity::{
	all::{CommandInteraction, CommandOptionType},
	builder::{CreateCommand, CreateCommandOption}
};
use serenity::all::{ChannelId, CreateInteractionResponse, CreateInteractionResponseMessage, MessageBuilder};
use serenity::prelude::Context;

use crate::{
	model::{
		error::level_request_error::LevelRequestError, level_request::LevelRequest,
		request_score::RequestRating
	},
	service::level_request_service::LevelRequestService
};

pub fn register() -> CreateCommand {
	CreateCommand::new("request-level")
		.description("Request a level to Ryder")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The ID of the level to request."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"request-rating",
				"The amount of Stars/Moons requested."
			)
			.required(true)
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
				"video-link",
			"A link to the video showcasing the requested level."
			).required(true)
		)
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
	let level_request = LevelRequest {
		discord_user_id: u64::from(command.user.id),
		level_id: command
			.data
			.options
			.get(0)
			.unwrap()
			.value
			.as_i64()
			.unwrap()
			.unsigned_abs(),
		request_score: RequestRating::from_str(
			command.data.options.get(1).unwrap().value.as_str().unwrap()
		)
		.unwrap(),
		youtube_video_link: command.data.options.get(2).unwrap().value.as_str().unwrap().to_string()
	};

	let service = LevelRequestService::new(level_request);
	let content: String;
	match service.request_level().await {
		Ok(level_data) => {
			content = "Level has been requested successfully!".to_string();
			invoke_ephermal(&content, &ctx, &command).await;

			let request_message = MessageBuilder::new()
				.push(format!("\"{}\" by {}\n", &level_data.level_name, &level_data.level_author))
				.push(format!("{}\n", &level_data.level_id))
				.push(format!("Requested {}\n", &level_data.request_score))
				.push(format!("{}", &level_data.youtube_video_link))
				.build();

			if let Err(error) = ChannelId::new(1193493680594616411).say(&ctx.http, &request_message).await {
				println!("Error sending message: {error:?}");
			}
		}
		Err(error) => match error {
			LevelRequestError::LevelRequestExists => {
				content = "Level has already been requested.".to_string();
				invoke_ephermal(&content, &ctx, &command).await;
			}
			LevelRequestError::MalformedRequestError => {
				content = "Level request was not properly formatted.".to_string();
				invoke_ephermal(&content, &ctx, &command).await;
			}
			LevelRequestError::RequestError => {
				content = "There was an error making the request.".to_string();
				invoke_ephermal(&content, &ctx, &command).await;
			}
			LevelRequestError::SerializeError => {
				content = "There was an error making the request.".to_string();
				invoke_ephermal(&content, &ctx, &command).await;
			}
			LevelRequestError::RequestXApiError => {
				content = "There was an error making the request.".to_string();
				invoke_ephermal(&content, &ctx, &command).await;
			}
		}
	}
}

async fn invoke_ephermal(content: &str, ctx: &Context, command: &CommandInteraction) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = command.create_response(&ctx.http, builder).await {
		println!("Cannot respond to slash command: {err}");
	}
}
