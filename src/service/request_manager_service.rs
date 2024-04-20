use crate::model::{
	request_manager::UpdateRequestManager,
	requestx_api::{
		error::level_request_error::LevelRequestError, requestx_api_client::RequestXApiClient
	}
};

pub struct RequestManagerService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> RequestManagerService<'a> {
	pub fn new() -> Self {
		RequestManagerService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn update_request_manager(
		&self,
		update_request_manager: &UpdateRequestManager
	) -> Result<(), LevelRequestError> {
		return self
			.requestx_api_client
			.make_update_request_manager_request(update_request_manager)
			.await;
	}
}
