use std::str::FromStr;

use log::error;
use serenity::{
	all::{ChannelId, CommandInteraction, CommandOptionType, MessageBuilder},
	builder::{CreateCommand, CreateCommandOption},
	prelude::Context
};

use crate::{
	config::client_config::CLIENT_CONFIG,
	model::{
		level_request::{
			GetLevelRequest, LevelRequest, UpdateLevelRequest, UpdateLevelRequestMessageId
		},
		request_score::RequestRating
	},
	service::level_request_service::LevelRequestService,
	util,
	util::discord::{invoke_ephermal, log_to_discord}
};

pub fn register_request_level() -> CreateCommand {
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
				"Request for reviewers to potentially review your request."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Boolean,
				"notify",
				"Notify when a review has been made or if the level has been sent."
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
			.unwrap(),
		notify: command
			.data
			.options
			.get(4)
			.unwrap()
			.value
			.as_bool()
			.unwrap()
	};

	let service = LevelRequestService::new();
	let content: &str;

	match service.request_level(level_request).await {
		Ok(level_request_data) => {
			content = "Level has been requested successfully!";
			invoke_ephermal(&content, &ctx, &command).await;

			match util::discord::send_level_request_message_to_discord(&ctx, &level_request_data)
				.await
			{
				Ok(message_data) => {
					let update_request_message_id = UpdateLevelRequestMessageId {
						level_id: level_request_data.level_id,
						discord_message_id: message_data.id.get()
					};
					if let Err(error) = &service
						.update_request_message_id(update_request_message_id)
						.await
					{
						error!("Error updating message ID: {error:?}");
					}
				}
				Err(send_message_error) => {
					error!("Error sending message: {send_message_error}");
				}
			}
			{
				let mut log_message = MessageBuilder::new();
				log_message.push_bold(format!("{} ", command.user.name));
				log_message.push_line(format!("({}) has requested a level", command.user.id));
				log_message.push_codeblock(format!("{:?}", &level_request_data), Some("rust"));
				log_to_discord(ctx.clone(), log_message.build()).await
			}
		}
		Err(error) => {
			invoke_ephermal(&error.to_string(), &ctx, &command).await;

			{
				let mut log_message = MessageBuilder::new();
				log_message.push_bold(format!("{} ", command.user.name));
				log_message.push_line(format!(
					"({}) caused an error when requesting a level",
					command.user.id
				));
				log_message.push_codeblock(format!("{:?}", error), Some("rust"));
				log_to_discord(ctx.clone(), log_message.build()).await
			}
		}
	}
}

pub fn register_edit_level_request() -> CreateCommand {
	CreateCommand::new("edit-level-request")
		.description("Edits an existing level request")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the level request to edit."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"request-rating",
				"The amount of Stars/Moons requested."
			)
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
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"request-feedback",
			"Request for reviewers to potentially review your request."
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"notify",
			"Notify when a review has been made or if the level has been sent."
		))
}

pub async fn run_edit_level_request(ctx: &Context, command: &CommandInteraction) {
	let update_level_request = UpdateLevelRequest {
		discord_user_id: command.user.id.get(),
		level_id: command
			.data
			.options
			.get(0)
			.unwrap()
			.value
			.as_i64()
			.unwrap()
			.unsigned_abs(),
		request_score: if let Some(request_rating_command_option) = command
			.data
			.options
			.iter()
			.find(|command_option| command_option.name.eq("request-rating"))
		{
			Some(
				RequestRating::from_str(request_rating_command_option.value.as_str().unwrap())
					.unwrap()
			)
		} else {
			None
		},
		youtube_video_link: if let Some(video_link_command_option) = command
			.data
			.options
			.iter()
			.find(|command_option| command_option.name.eq("video-link"))
		{
			Some(
				video_link_command_option
					.value
					.as_str()
					.unwrap()
					.to_string()
			)
		} else {
			None
		},
		has_requested_feedback: if let Some(has_requested_feedback_command_option) = command
			.data
			.options
			.iter()
			.find(|command_option| command_option.name.eq("request-feedback"))
		{
			Some(
				has_requested_feedback_command_option
					.value
					.as_bool()
					.unwrap()
			)
		} else {
			None
		},
		notify: if let Some(notify_command_option) = command
			.data
			.options
			.iter()
			.find(|command_option| command_option.name.eq("notify"))
		{
			Some(notify_command_option.value.as_bool().unwrap())
		} else {
			None
		}
	};

	let service = LevelRequestService::new();

	match service.update_level_request(update_level_request).await {
		Ok(level_request_data) => {
			if let Err(edit_message_error) =
				util::discord::send_level_request_message_to_discord(&ctx, &level_request_data)
					.await
			{
				error!(
					"Unable to edit level request message: {}",
					edit_message_error
				);
				return;
			}

			{
				let mut log_message = MessageBuilder::new();
				log_message.push_bold(format!("{} ", command.user.name));
				log_message.push_line(format!("({}) has edited a level request", command.user.id));
				log_message.push_codeblock(format!("{:?}", &level_request_data), Some("rust"));
				log_to_discord(ctx.clone(), log_message.build()).await
			}

			let content = "Level request has been edited successfully!".to_string();
			invoke_ephermal(&content, &ctx, &command).await;
		}
		Err(error) => {
			invoke_ephermal(&error.to_string(), &ctx, &command).await;

			{
				let mut log_message = MessageBuilder::new();
				log_message.push_bold(format!("{} ", command.user.name));
				log_message.push_line(format!(
					"({}) caused an error when editing a level request",
					command.user.id
				));
				log_message.push_codeblock(format!("{:?}", error), Some("rust"));
				log_to_discord(ctx.clone(), log_message.build()).await
			}
		}
	}
}

pub fn register_delete_level_request() -> CreateCommand {
	CreateCommand::new("delete-level-request")
		.description("Deletes an existing level request")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the level request to delete."
			)
			.required(true)
		)
}

pub async fn run_delete_level_request(ctx: &Context, command: &CommandInteraction) {
	if !command.user.id.eq(&CLIENT_CONFIG.discord_bot_admin_id) {
		invoke_ephermal("Forbidden", &ctx, &command).await;
		return;
	}

	let delete_level_request = GetLevelRequest {
		level_id: command
			.data
			.options
			.get(0)
			.unwrap()
			.value
			.as_i64()
			.unwrap()
			.unsigned_abs()
	};

	let service = LevelRequestService::new();

	match service.delete_level_request(delete_level_request).await {
		Ok(level_request_data) => {
			if let Err(delete_message_error) =
				ChannelId::new(CLIENT_CONFIG.discord_requests_channel_id)
					.delete_message(&ctx.http, level_request_data.discord_message_id.unwrap())
					.await
			{
				error!(
					"Unable to delete level request message: {}",
					delete_message_error
				);
				return;
			}
			if let Some(discord_thread_id) = level_request_data.discord_thread_id {
				if let Err(delete_message_error) =
					ChannelId::new(discord_thread_id).delete(&ctx.http).await
				{
					error!(
						"Unable to delete level request thread: {}",
						delete_message_error
					);
					return;
				}
			}

			{
				let mut log_message = MessageBuilder::new();
				log_message.push_bold(format!("{} ", command.user.name));
				log_message.push_line(format!("({}) has deleted a level request", command.user.id));
				log_message.push_codeblock(format!("{:?}", &level_request_data), Some("rust"));
				log_to_discord(ctx.clone(), log_message.build()).await
			}

			let content = "Level request has been deleted successfully!".to_string();
			invoke_ephermal(&content, &ctx, &command).await;
		}
		Err(error) => {
			invoke_ephermal(&error.to_string(), &ctx, &command).await;

			{
				let mut log_message = MessageBuilder::new();
				log_message.push_bold(format!("{} ", command.user.name));
				log_message.push_line(format!(
					"({}) cause an error when deleting a level request",
					command.user.id
				));
				log_message.push_codeblock(format!("{:?}", error), Some("rust"));
				log_to_discord(ctx.clone(), log_message.build()).await
			}
		}
	}
}
