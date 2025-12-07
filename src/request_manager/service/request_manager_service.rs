use crate::{
	level_request::model::level_request_error::LevelRequestError,
	request_manager::model::request_manager::RequestConfig,
	requestx_api::requestx_api_client::RequestXApiClient,
};

pub struct RequestConfigService<'a> {
	requestx_api_client: RequestXApiClient<'a>,
}

impl<'a> RequestConfigService<'a> {
	pub fn new() -> Self {
		RequestConfigService {
			requestx_api_client: RequestXApiClient::new(),
		}
	}

	pub async fn get_request_config(&self) -> Result<RequestConfig, LevelRequestError> {
		self.requestx_api_client.get_request_config().await
	}

	pub async fn update_request_config(
		&self,
		update_request_manager: &RequestConfig,
	) -> Result<(), LevelRequestError> {
		self.requestx_api_client
			.update_request_manager(update_request_manager)
			.await
	}
}
