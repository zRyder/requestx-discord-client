use log::error;

use crate::{
	requestx_api::requestx_api_client::RequestXApiClient,
	send_level::model::{
		moderator::{SendLevelRequest, SentLevel},
		send_level_error::ModeratorError
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
		send_level_request: SendLevelRequest
	) -> Result<SentLevel, ModeratorError> {
		match self
			.requestx_api_client
			.send_level(send_level_request)
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
