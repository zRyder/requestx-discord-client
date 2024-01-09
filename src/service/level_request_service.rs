use crate::model::{
	error::level_request_error::LevelRequestError,
	level_request::{
		GetLevelRequest, LevelRequest, UpdateLevelRequestMessageId,
		UpdateLevelRequestThreadId
	},
	requestx_api::{
		level_request_data::LevelRequestData,
		requestx_api_client::RequestXApiClient
	}
};

pub struct LevelRequestService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> LevelRequestService<'a> {
	pub fn new() -> Self {
		LevelRequestService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn get_level_request(
		&self,
		get_level_request: GetLevelRequest
	) -> Result<Option<LevelRequestData>, LevelRequestError> {
		match self
			.requestx_api_client
			.get_level_request(get_level_request)
			.await
		{
			Ok(resp) => Ok(resp),
			Err(error) => Err(error)
		}
	}

	pub async fn request_level(
		&self,
		level_request: LevelRequest
	) -> Result<LevelRequestData, LevelRequestError> {
		match self
			.requestx_api_client
			.make_requestx_api_level_request(level_request)
			.await
		{
			Ok(resp) => Ok(resp),
			Err(error) => Err(error)
		}
	}

	pub async fn update_request_message_id(
		&self,
		update_level_request_message: UpdateLevelRequestMessageId
	) -> Result<(), LevelRequestError> {
		match self
			.requestx_api_client
			.update_request_message_id(update_level_request_message)
			.await
		{
			Ok(()) => Ok(()),
			Err(error) => Err(error)
		}
	}

	pub async fn update_request_thread_id(
		&self,
		update_level_thread_message: UpdateLevelRequestThreadId
	) -> Result<(), LevelRequestError> {
		match self
			.requestx_api_client
			.update_request_thread_id(update_level_thread_message)
			.await
		{
			Ok(()) => Ok(()),
			Err(error) => Err(error)
		}
	}
}
