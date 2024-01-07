use crate::model::{
	error::level_request_error::LevelRequestError, level_request::LevelRequest,
	request_score::RequestRating, requestx_api::requestx_api_client::RequestXApiClient
};
use crate::model::requestx_api::level_request_data::LevelRequestData;

pub struct LevelRequestService<'a> {
	requestx_api_client: RequestXApiClient<'a>,
	level_request: LevelRequest
}

impl<'a> LevelRequestService<'a> {
	pub fn new(level_request: LevelRequest) -> Self {
		LevelRequestService {
			requestx_api_client: RequestXApiClient::new(),
			level_request
		}
	}

	pub async fn request_level(self) -> Result<LevelRequestData, LevelRequestError> {
		match self
			.requestx_api_client
			.make_requestx_api_level_request(self.level_request)
			.await
		{
			Ok(resp) => Ok(resp),
			Err(error) => Err(error)
		}
	}
}
