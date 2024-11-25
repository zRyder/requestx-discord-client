use log::error;
use serenity::all::{
	ChannelId, CommandInteraction, Context, EditMessage, Mentionable, MessageBuilder, UserId
};

use crate::{
	config::client_config::CLIENT_CONFIG,
	model::{
		level_request::{GetLevelRequest, GetLevelReview},
		level_review::LevelReview,
		requestx_api::{
			error::{level_request_error::LevelRequestError, level_review_error::LevelReviewError},
			level_review_data::LevelReviewData,
			requestx_api_client::RequestXApiClient
		}
	},
	service::level_request_service::LevelRequestService
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
		review_contents: &str
	) -> Result<String, LevelReviewError> {
		let get_level_request = GetLevelRequest { level_id };
		let level_request_service = LevelRequestService::new();
		match level_request_service
			.get_level_request(get_level_request)
			.await
		{
			Ok(Some(level_request)) => {
				if !level_request.has_requested_feedback
					&& reviewer_discord_user_id.ne(&CLIENT_CONFIG.discord_bot_admin_id)
				{
					return Ok("The user has not requested feedback for this level".to_string());
				}

				let review_discord_message_id;
				let mut review_message = MessageBuilder::new();
				review_message
					.push_bold_line(format!("Review by {}", command.user.id.mention()))
					.push_line("")
					.push_quote_line_safe(review_contents);

				if level_request.notify {
					review_message.push_line("");
					review_message.push_line(format!(
						"{}",
						UserId::new(level_request.discord_id).mention()
					));
				}

				let get_level_review = GetLevelReview {
					discord_user_id: reviewer_discord_user_id,
					level_id
				};

				match self.get_level_review(get_level_review).await {
					Ok(Some(existing_level_review)) => {
						if let Err(edit_message_error) =
							ChannelId::new(level_request.discord_message_id.unwrap())
								.edit_message(
									&ctx.http,
									existing_level_review.discord_message_id.unwrap(),
									EditMessage::new().content(&review_message.build())
								)
								.await
						{
							error!("Unable to edit review message: {}", edit_message_error);
							return Err(LevelReviewError::RequestError);
						};
						review_discord_message_id =
							existing_level_review.discord_message_id.unwrap();
					}
					Ok(None) => {
						match ChannelId::new(level_request.discord_message_id.unwrap())
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
					}
					Err(level_review_error) => {
						error!("Error getting level review: {}", level_review_error);
						return Err(level_review_error);
					}
				}

				let level_review = LevelReview {
					discord_user_id: reviewer_discord_user_id,
					discord_message_id: review_discord_message_id,
					level_id,
					review_contents: review_contents.to_string()
				};
				if let Err(save_level_review_error) = self.post_level_review(&level_review).await {
					Err(save_level_review_error)
				} else {
					Ok("Review submitted".to_string())
				}
			}
			Ok(None) => {
				error!("Level request with ID {} does not exist", level_id);
				Err(LevelReviewError::LevelRequestDoesNotExists)
			}
			Err(error) => match error {
				LevelRequestError::RequestError => Err(LevelReviewError::RequestError),
				LevelRequestError::SerializeError(_field) => Err(LevelReviewError::RequestError),
				LevelRequestError::RequestXApiError(error_message) => {
					Err(LevelReviewError::RequestXApiError(error_message))
				}
				_ => {
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
