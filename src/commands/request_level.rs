use std::str::FromStr;

use serenity::{
	all::{CommandInteraction, CommandOptionType},
	builder::{CreateCommand, CreateCommandOption}
};

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
		.add_option(CreateCommandOption::new(
			CommandOptionType::String,
			"video-link",
			"A link to the video showcasing the requested level."
		))
}

pub async fn run(command: &CommandInteraction) -> String {
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
		video_link: if let Some(link) = command.data.options.get(2).unwrap().value.as_str() {
			Some(link.to_string())
		} else {
			None
		}
	};

	let service = LevelRequestService::new(level_request);
	match service.request_level().await {
		Ok(_) => "Level has been requested successfully!".to_string(),
		Err(error) => match error {
			LevelRequestError::LevelRequestExists => {
				"Level has already been requested.".to_string()
			}
			LevelRequestError::MalformedRequestError => {
				"Level request was not properly formatted.".to_string()
			}
			LevelRequestError::RequestError => "There was an error making the request.".to_string(),
			LevelRequestError::SerializeError => {
				"There was an error making the request.".to_string()
			}
			LevelRequestError::RequestXApiError => {
				"There was an error making the request.".to_string()
			}
		}
	}
}
