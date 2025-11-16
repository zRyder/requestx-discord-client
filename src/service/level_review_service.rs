use log::error;

use crate::{
	config::client_config::CLIENT_CONFIG,
	model::{
		error::level_review_error::LevelReviewError,
		level_request::LevelRequest,
		level_review::{LevelReview, UpdateLevelReviewMessageId},
		requestx_api::requestx_api_client::RequestXApiClient
	}
};

pub struct LevelReviewService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

const MAX_REVIEW_CHARACTERS: usize = 4000;

impl<'a> LevelReviewService<'a> {
	pub fn new() -> Self {
		LevelReviewService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn get_level_review(
		&self,
		reviewer_discord_id: u64,
		level_id: u64
	) -> Result<Option<LevelReview>, LevelReviewError> {
		self.requestx_api_client
			.get_level_review(reviewer_discord_id, level_id)
			.await
	}

	pub async fn review_level(
		&self,
		level_review: &LevelReview
	) -> Result<LevelRequest, LevelReviewError> {
		Self::validate_level_review(&level_review.review_contents)?;

		let level_request = self
			.requestx_api_client
			.get_level_request(level_review.level_id)
			.await
			.map_err(|get_level_request_error| {
				error!("Error getting level request: {:?}", get_level_request_error);
				LevelReviewError::from(get_level_request_error)
			})?
			.map_or_else(
				|| {
					error!("Level request does not exist");
					Err(LevelReviewError::LevelRequestDoesNotExists)
				},
				|level_request_data| Ok(LevelRequest::from(level_request_data))
			)?;

		if !level_request.has_requested_feedback
			&& level_review
				.discord_user_id
				.ne(&CLIENT_CONFIG.discord_bot_admin_id)
		{
			return Err(LevelReviewError::UserHasNotRequestedFeedback);
		}

		if let Err(save_level_review_error) = self
			.requestx_api_client
			.create_level_review(&level_review)
			.await
		{
			error!("Error saving level review: {:?}", save_level_review_error);
			return Err(save_level_review_error);
		}

		Ok(level_request)
	}

	pub async fn update_level_review_message_id(
		&self,
		update_level_review_message: UpdateLevelReviewMessageId
	) -> Result<(), LevelReviewError> {
		self.requestx_api_client
			.update_level_review_message_id(update_level_review_message)
			.await
	}

	fn validate_level_review(review_contents: &str) -> Result<(), LevelReviewError> {
		if review_contents.len() > MAX_REVIEW_CHARACTERS {
			return Err(LevelReviewError::DiscordFormattingError(
				0,
				review_contents.to_string()
			));
		}

		for (index, paragraph) in review_contents
			.lines()
			.filter(|line| !line.trim().is_empty())
			.enumerate()
		{
			if paragraph.len() > 1024 {
				return Err(LevelReviewError::DiscordFormattingError(
					index,
					paragraph.to_string()
				));
			}
		}

		Ok(())
	}
}
