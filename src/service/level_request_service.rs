use crate::model::{
	error::level_request_error::LevelRequestError, level_request::LevelRequest,
	request_score::RequestRating, requestx_api::requestx_api_client::RequestXApiClient
};

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

	pub async fn request_level(self) -> Result<(), LevelRequestError> {
		if (self.level_request.request_score == RequestRating::Ten
			&& self.level_request.video_link.is_none())
		{
			Err(LevelRequestError::MalformedRequestError)
		} else {
			match self
				.requestx_api_client
				.make_requestx_api_level_request(self.level_request)
				.await
			{
				Ok(()) => Ok(()),
				Err(error) => Err(error)
			}
		}
	}
}
