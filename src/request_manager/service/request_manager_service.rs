use crate::{
	level_request::model::level_request_error::LevelRequestError,
	request_manager::model::request_manager::UpdateRequestManagerRequest,
	requestx_api::requestx_api_client::RequestXApiClient,
};

pub struct RequestManagerService<'a> {
	requestx_api_client: RequestXApiClient<'a>,
}

impl<'a> RequestManagerService<'a> {
	pub fn new() -> Self {
		RequestManagerService {
			requestx_api_client: RequestXApiClient::new(),
		}
	}

	pub async fn update_request_manager(
		&self,
		update_request_manager: &UpdateRequestManagerRequest,
	) -> Result<(), LevelRequestError> {
		self.requestx_api_client
			.update_request_manager(update_request_manager)
			.await
	}
}
