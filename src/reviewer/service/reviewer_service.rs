use crate::{
	requestx_api::requestx_api_client::RequestXApiClient,
	reviewer::model::{reviewer::CreateReviewerRequest, reviewer_error::ReviewerError}
};

pub struct ReviewerService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> ReviewerService<'a> {
	pub fn new() -> Self {
		ReviewerService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn create_reviewer(&self, discord_id: u64) -> Result<(), ReviewerError> {
		let create_reviewer_request = CreateReviewerRequest {
			reviewer_discord_id: discord_id
		};

		self.requestx_api_client
			.create_reviewer(create_reviewer_request)
			.await
	}

	pub async fn remove_reviewer(&self, discord_id: u64) -> Result<(), ReviewerError> {
		self.requestx_api_client.remove_reviewer(discord_id).await
	}
}
