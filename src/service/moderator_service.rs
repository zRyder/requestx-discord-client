use log::error;

use crate::model::{
	moderator::Moderator,
	requestx_api::{
		level_request_data::LevelRequestData, moderator_data::ModeratorError,
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
	) -> Result<LevelRequestData, ModeratorError> {
		match self
			.requestx_api_client
			.make_send_level_request(send_level_request)
			.await
		{
			Ok(level_request_data) => Ok(level_request_data),
			Err(send_level_error) => {
				error!("Error sending level: {}", send_level_error);
				Err(send_level_error)
			}
		}
	}
}
