use std::str::FromStr;

use serenity::{
	all::{ChannelId, CommandInteraction, CommandOptionType, MessageBuilder},
	builder::{CreateCommand, CreateCommandOption},
	prelude::Context
};

use crate::{
	model::{
		error::level_request_error::LevelRequestError,
		level_request::{LevelRequest, UpdateLevelRequestMessageId},
		request_score::RequestRating
	},
	service::level_request_service::LevelRequestService,
	util::discord::invoke_ephermal
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
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Boolean,
				"request-feedback",
				"Set this to true if you would like a reviewer to potentially review your request."
			)
				.required(true)
		)
}

pub async fn run_request_level(ctx: &Context, command: &CommandInteraction) {
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
		youtube_video_link: command
			.data
			.options
			.get(2)
			.unwrap()
			.value
			.as_str()
			.unwrap()
			.to_string(),
		has_requested_feedback: command
			.data
			.options
			.get(3)
			.unwrap()
			.value
			.as_bool()
			.unwrap()
	};

	let service = LevelRequestService::new();
	let content: String;
	match service.request_level(level_request).await {
		Ok(level_data) => {
			content = "Level has been requested successfully!".to_string();
			invoke_ephermal(&content, &ctx, &command).await;

			let mut request_message = MessageBuilder::new();
			request_message
				.push_line(format!(
					"\"{}\" by {}",
					&level_data.level_name, &level_data.level_author
				))
				.push_line(format!("{}", &level_data.level_id))
				.push_line(format!("Requested {}", &level_data.request_score));
			if level_data.has_requested_feedback {
				request_message.push_line("Feedback has been requested!");
			}
				request_message.push_line(format!("{}", &level_data.youtube_video_link));

			match ChannelId::new(1193493680594616411)
				.say(&ctx.http, &request_message.build())
				.await
			{
				Ok(msg) => {
					let update_request_message_id = UpdateLevelRequestMessageId {
						level_id: level_data.level_id,
						discord_message_id: msg.id.get()
					};
					if let Err(error) = &service
						.update_request_message_id(update_request_message_id)
						.await
					{
						println!("Error updating message ID: {error:?}");
					}
				}
				Err(error) => {
					println!("Error sending message: {error:?}");
				}
			}
		}
		Err(error) => match error {
			LevelRequestError::LevelRequestExists => {
				content = "Level has already been requested.".to_string();
				invoke_ephermal(&content, &ctx, &command).await;
			}
			LevelRequestError::LevelRequestDoesNotExists => {
				content = "Level request does not exist.".to_string();
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
