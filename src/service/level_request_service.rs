use log::warn;

use crate::{
	config::constants::YOUTUBE_LINK_REGEX,
	model::{
		level_request::{
			GetLevelRequest, LevelRequest, UpdateLevelRequest, UpdateLevelRequestMessageId,
			UpdateLevelRequestThreadId
		},
		requestx_api::{
			error::level_request_error::LevelRequestError, level_request_data::LevelRequestData,
			requestx_api_client::RequestXApiClient
		}
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
		if !Self::is_valid_youtube_link(&level_request.youtube_video_link) {
			warn!("Invalid link: {}", &level_request.youtube_video_link);
			return Err(LevelRequestError::SerializeError(
				level_request.youtube_video_link
			));
		}
		match self
			.requestx_api_client
			.make_requestx_api_level_request(level_request)
			.await
		{
			Ok(level_request_data) => Ok(level_request_data),
			Err(error) => Err(error)
		}
	}

	pub async fn update_level_request(
		&self,
		level_request: UpdateLevelRequest
	) -> Result<LevelRequestData, LevelRequestError> {
		if let Some(youtube_video_link) = &level_request.youtube_video_link {
			if !Self::is_valid_youtube_link(youtube_video_link) {
				warn!("Invalid link: {}", &youtube_video_link);
				return Err(LevelRequestError::SerializeError(
					youtube_video_link.clone()
				));
			}
		}
		match self
			.requestx_api_client
			.make_requestx_api_update_level_request(level_request)
			.await
		{
			Ok(level_request_data) => Ok(level_request_data),
			Err(error) => Err(error)
		}
	}

	pub async fn delete_level_request(
		&self,
		level_request: GetLevelRequest
	) -> Result<LevelRequestData, LevelRequestError> {
		match self
			.requestx_api_client
			.make_requestx_api_delete_level_request(level_request)
			.await
		{
			Ok(level_request_data) => Ok(level_request_data),
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

	fn is_valid_youtube_link(youtube_link: &str) -> bool {
		let regex = regex::RegexBuilder::new(YOUTUBE_LINK_REGEX)
			.case_insensitive(true)
			.multi_line(true)
			.build()
			.unwrap();

		regex.is_match(youtube_link)
	}
}
