use crate::model::{
	error::level_request_error::LevelRequestError,
	level_request::{GetLevelReview, UpdateLevelRequestMessageId},
	level_review::{LevelReview, UpdateLevelReviewMessageId},
	requestx_api::{
		level_review_data::LevelReviewData, level_review_error::LevelReviewError,
		requestx_api_client::RequestXApiClient
	}
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

	pub async fn update_review_message_id(
		&self,
		update_level_review_message: UpdateLevelReviewMessageId
	) -> Result<(), LevelReviewError> {
		match self
			.requestx_api_client
			.update_review_message_id(update_level_review_message)
			.await
		{
			Ok(resp) => Ok(()),
			Err(error) => Err(error)
		}
	}
}
