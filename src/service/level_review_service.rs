use log::error;
use serenity::all::{
	ChannelId, CommandInteraction, Context, EditMessage, Mentionable, MessageBuilder, UserId
};

use crate::{
	config::client_config::CLIENT_CONFIG,
	model::{
		error::level_request_error::LevelRequestError,
		level_request::{GetLevelRequest, GetLevelReview, UpdateLevelRequestThreadId},
		level_review::LevelReview,
		requestx_api::{
			level_review_data::LevelReviewData, level_review_error::LevelReviewError,
			requestx_api_client::RequestXApiClient
		}
	},
	service::level_request_service::LevelRequestService,
	util::discord::create_thread
};

pub struct LevelReviewService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> LevelReviewService<'a> {
	pub fn new() -> Self {
		LevelReviewService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn get_level_review(
		&self,
		get_level_review: GetLevelReview
	) -> Result<Option<LevelReviewData>, LevelReviewError> {
		match self
			.requestx_api_client
			.get_level_review(get_level_review)
			.await
		{
			Ok(resp) => Ok(resp),
			Err(error) => Err(error)
		}
	}

	pub async fn review_level(
		&self,
		ctx: &Context,
		command: &CommandInteraction,
		level_id: u64,
		reviewer_discord_user_id: u64,
		review_contents: String
	) -> Result<String, LevelReviewError> {
		let get_level_request = GetLevelRequest { level_id };
		let level_request_service = LevelRequestService::new();
		match level_request_service
			.get_level_request(get_level_request)
			.await
		{
			Ok(potential_level_request) => {
				if let Some(level_request) = potential_level_request {
					if !level_request.has_requested_feedback
						&& reviewer_discord_user_id.ne(&CLIENT_CONFIG.discord_bot_admin_id)
					{
						return Ok("The user has not requested feedback for this level".to_string());
					}
					if let Some(level_request_message_id) = level_request.discord_message_id {
						// Request Message Exists
						let get_level_review = GetLevelReview {
							discord_user_id: reviewer_discord_user_id,
							level_id
						};
						match self.get_level_review(get_level_review).await {
							Ok(potential_level_review) => {
								let thread_id;
								if let Some(thread) = level_request.discord_thread_id {
									thread_id = thread;
								} else {
									if let Ok(thread) = create_thread(
										&ctx,
										&command,
										level_request_message_id,
										&level_request
									)
									.await
									{
										thread_id = thread;

										let update_level_request_thread_id =
											UpdateLevelRequestThreadId {
												level_id: level_request.level_id,
												discord_thread_id: thread_id
											};

										if let Err(update_level_request_thread_id_error) =
											level_request_service
												.update_request_thread_id(
													update_level_request_thread_id
												)
												.await
										{
											error!(
												"Unable to update level request thread ID: {}",
												update_level_request_thread_id_error
											);
											return Err(LevelReviewError::RequestError);
										}
									} else {
										return Err(LevelReviewError::RequestError);
									}
								}

								let mut review_message = MessageBuilder::new();
								review_message
									.push_bold_line(format!(
										"Review by {}",
										command.user.id.mention()
									))
									.push_line("")
									.push_quote_line_safe(&review_contents);

								if level_request.notify {
									review_message.push_line("");
									review_message.push_line(format!(
										"{}",
										UserId::new(level_request.discord_id).mention()
									));
								}

								let review_discord_message_id: u64;
								if let Some(existing_level_review) = potential_level_review {
									// EXISTING LEVEL REVIEW
									if let Some(review_message_id) =
										existing_level_review.discord_message_id
									{
										review_discord_message_id = review_message_id;
										if let Err(edit_message_error) = ChannelId::new(thread_id)
											.edit_message(
												&ctx.http,
												review_message_id,
												EditMessage::new().content(&review_message.build())
											)
											.await
										{
											error!(
												"Unable to edit review message: {}",
												edit_message_error
											);
											return Err(LevelReviewError::RequestError);
										};
									} else {
										// There is probably a database inconsistency if this
										// happens
										return Err(LevelReviewError::RequestError);
									}
								} else {
									match ChannelId::new(thread_id)
										.say(&ctx.http, &review_message.build())
										.await
									{
										Ok(message) => review_discord_message_id = message.id.get(),
										Err(send_level_review_error) => {
											error!(
												"Unable to send level review to Discord: {}",
												send_level_review_error
											);
											return Err(LevelReviewError::RequestError);
										}
									};
								};

								let level_review = LevelReview {
									discord_user_id: reviewer_discord_user_id,
									discord_message_id: review_discord_message_id,
									level_id,
									review_contents: review_contents.clone()
								};
								if let Err(save_level_review_error) =
									self.post_level_review(&level_review).await
								{
									Err(save_level_review_error)
								} else {
									Ok("Review submitted".to_string())
								}
							}
							Err(level_review_error) => Err(level_review_error)
						}
					} else {
						Err(LevelReviewError::RequestError)
					}
				} else {
					Err(LevelReviewError::LevelRequestDoesNotExists)
				}
			}
			Err(error) => match error {
				LevelRequestError::RequestError => Err(LevelReviewError::RequestError),
				LevelRequestError::SerializeError => Err(LevelReviewError::RequestError),
				LevelRequestError::RequestXApiError => Err(LevelReviewError::RequestXApiError),
				LevelRequestError::LevelRequestExists => {
					unreachable!()
				}
			}
		}
	}

	async fn post_level_review(
		&self,
		level_review: &LevelReview
	) -> Result<LevelReviewData, LevelReviewError> {
		match self
			.requestx_api_client
			.make_requestx_api_level_review_request(level_review)
			.await
		{
			Ok(resp) => Ok(resp),
			Err(error) => Err(error)
		}
	}
}
