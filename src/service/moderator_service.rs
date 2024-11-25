use log::error;

use crate::model::{
	moderator::Moderator,
	requestx_api::{
		moderator_data::{ModeratorError, SendLevelData},
		requestx_api_client::RequestXApiClient
	}
};

pub struct ModeratorService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> ModeratorService<'a> {
	pub fn new() -> Self {
		ModeratorService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn send_level(
		&self,
		send_level_request: Moderator
	) -> Result<SendLevelData, ModeratorError> {
		match self
			.requestx_api_client
			.make_send_level_request(send_level_request)
			.await
		{
			Ok(send_level_data) => Ok(send_level_data),
			Err(send_level_error) => {
				error!("Error sending level: {}", send_level_error);
				Err(send_level_error)
			}
		}
	}
}
