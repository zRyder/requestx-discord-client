use std::future::Future;

use serenity::{
	all::{
		ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand,
		CreateCommandOption, EditMessage, MessageBuilder, MessageId, Route::ChannelMessageThreads
	},
	builder::CreateThread,
	futures::future::err,
	prelude::Mentionable,
	Error
};

use crate::{
	model::{
		error::level_request_error::LevelRequestError,
		level_request::{
			GetLevelRequest, GetLevelReview, UpdateLevelRequestMessageId,
			UpdateLevelRequestThreadId
		},
		level_review::{LevelReview, UpdateLevelReviewMessageId},
		requestx_api::{
			level_request_data::LevelRequestData, level_review_data::LevelReviewData,
			level_review_error::LevelReviewError
		}
	},
	serenity::common::invoke_ephermal,
	service::{
		level_request_service::LevelRequestService, level_review_service::LevelReviewService
	}
};

pub fn register() -> CreateCommand {
	CreateCommand::new("review")
		.description("Submit a review for the given level")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the request to review."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"review-contents",
				"The review to be shared with the Discord user who requested the level."
			)
			.required(true)
		)
}

pub async fn run(ctx: &Context, command: &CommandInteraction) {
	let mut content: String;
	if !command
		.user
		.has_role(&ctx.http, 1192954008013385839, 1192955803418775612)
		.await
		.unwrap()
	{
		content = "Unauthorized".to_string();
		invoke_ephermal(&content, &ctx, &command).await;
	} else {
		let mut update_flag = false;
		let discord_user_id = command.user.id.get();
		let level_id = command
			.data
			.options
			.get(0)
			.unwrap()
			.value
			.as_i64()
			.unwrap()
			.unsigned_abs();
		let review_contents = command
			.data
			.options
			.get(1)
			.unwrap()
			.value
			.as_str()
			.unwrap()
			.to_string();

		let get_level_request = GetLevelRequest {
			discord_user_id,
			level_id
		};
		let level_request_service = LevelRequestService::new();
		match level_request_service
			.get_level_request(get_level_request)
			.await
		{
			Ok(level) => {
				if let Some(message_id) = level.discord_message_id {
					let level_review_service = LevelReviewService::new();
					let get_level_review_request = GetLevelReview {
						discord_user_id,
						level_id
					};
					match level_review_service
						.get_level_review(get_level_review_request)
						.await
					{
						Ok(potential_level_review_data) => {
							let mut thread_id;
							let mut level_review = LevelReview {
								discord_user_id: u64::from(command.user.id),
								discord_message_id: None,
								level_id: command
									.data
									.options
									.get(0)
									.unwrap()
									.value
									.as_i64()
									.unwrap()
									.unsigned_abs(),
								review_contents: review_contents.clone()
							};
							let mut review_message = MessageBuilder::new();
							review_message
								.push_bold_line(format!("Review by {}", command.user.id.mention()));
							if let Some(thread) = level.discord_thread_id {
								thread_id = thread;
							} else {
								if let Ok(thread) =
									create_thread(&ctx, &command, message_id, &level).await
								{
									thread_id = thread;
								} else {
									content = "Error creating new thread...".to_string();
									invoke_ephermal(&content, &ctx, &command).await;
									thread_id = 0;
								}
							}

							if let Some(level_review_data) = potential_level_review_data {
								// Eisting review
								if let Some(review_message_id) =
									level_review_data.discord_message_id
								{
									level_review.discord_message_id = Some(review_message_id);
									level_review.review_contents = review_contents.clone();
									review_message.push_quote_line_safe(&review_contents);
									// EDIT REVIEW MESSAGE
									match ChannelId::new(thread_id)
										.edit_message(
											&ctx.http,
											level_review_data.discord_message_id.unwrap(),
											EditMessage::new().content(&review_message.build())
										)
										.await
									{
										Ok(msg) => {}
										Err(error) => {
											println!("{}", error);
											content = "Unable to write review...27".to_string();
											invoke_ephermal(&content, &ctx, &command).await;
										}
									}
								}
							} else {
								review_message.push_quote_line_safe(&review_contents);
								match ChannelId::new(thread_id)
									.say(&ctx.http, &review_message.build())
									.await
								{
									Ok(msg) => {
										level_review.discord_message_id = Some(msg.id.get());
										update_flag = true;

										let update_level_request_thread_id =
											UpdateLevelRequestThreadId {
												discord_user_id: level.discord_id,
												level_id: level.level_id,
												discord_thread_id: thread_id
											};
										if let Err(error) = level_request_service
											.update_request_thread_id(
												update_level_request_thread_id
											)
											.await
										{
											content =
												"Unable to update request thread ID...".to_string();
											invoke_ephermal(&content, &ctx, &command).await;
										}
									}
									Err(error) => {
										println!("{}", error);
										content = "Unable to write review...4".to_string();
										invoke_ephermal(&content, &ctx, &command).await;
									}
								}
							}

							match level_review_service.review_level(&level_review).await {
								Ok(level_review_data) => {
									if update_flag {
										let update_level_review_message_id =
											UpdateLevelReviewMessageId {
												discord_user_id: level_review.discord_user_id,
												level_id: level_review.level_id,
												discord_message_id: level_review
													.discord_message_id
													.unwrap()
											};
										if let Err(error) = level_review_service
											.update_review_message_id(
												update_level_review_message_id
											)
											.await
										{
											content = "Error updaing review...".to_string();
											invoke_ephermal(&content, &ctx, &command).await;
										}
									}

									content = "Review submitted".to_string();
									invoke_ephermal(&content, &ctx, &command).await;
								}
								Err(error) => {
									println!("{}", error);
									content = "Server failed to save review".to_string();
									invoke_ephermal(&content, &ctx, &command).await;
								}
							}
						}
						Err(error) => match error {
							LevelReviewError::LevelRequestDoesNotExists => {
								content = "Level request does not exist.".to_string();
								invoke_ephermal(&content, &ctx, &command).await;
							}
							LevelReviewError::RequestError => {
								content = "Error making request".to_string();
								invoke_ephermal(&content, &ctx, &command).await;
							}
							LevelReviewError::SerializeError => {
								content = "Error serializing".to_string();
								invoke_ephermal(&content, &ctx, &command).await;
							}
							LevelReviewError::RequestXApiError => {
								content = "Error calling API".to_string();
								invoke_ephermal(&content, &ctx, &command).await;
							}
						}
					}
				} else {
					content = "Couldn't find message ID for request...1 ".to_string();
					invoke_ephermal(&content, &ctx, &command).await;
				}
			}
			Err(error) => match error {
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
				LevelRequestError::LevelRequestExists => {
					unreachable!()
				}
			}
		}
	}
}

async fn create_thread(
	ctx: &Context,
	command: &CommandInteraction,
	message_id: u64,
	level: &LevelRequestData
) -> Result<u64, Error> {
	match ChannelId::new(1193493680594616411)
		.create_thread_from_message(
			&ctx.http,
			message_id,
			CreateThread::new(format!(
				"Reviews for: \"{}\" ({})",
				level.level_name, level.level_id
			))
			.audit_log_reason(&*format!(
				"Created via request command by: {} {}",
				command.user.name, command.user.id
			))
			.invitable(false)
		)
		.await
	{
		Ok(thread_channel_id) => Ok(thread_channel_id.id.get()),
		Err(error) => Err(error)
	}
}
