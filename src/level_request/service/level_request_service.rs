use log::warn;

use crate::level_request::model::level_request::GdLevelInfo;
use crate::{
	config::constants::YOUTUBE_LINK_REGEX,
	level_request::model::{
		level_request::{LevelRequest, UpdateLevelRequest, UpdateLevelRequestMessageId},
		level_request_error::LevelRequestError,
	},
	requestx_api::requestx_api_client::RequestXApiClient,
};

pub struct LevelRequestService<'a> {
	requestx_api_client: RequestXApiClient<'a>,
}

impl<'a> LevelRequestService<'a> {
	pub fn new() -> Self {
		LevelRequestService {
			requestx_api_client: RequestXApiClient::new(),
		}
	}

	pub async fn get_gd_level_info(
		&self,
		level_id: u64,
	) -> Result<Option<GdLevelInfo>, LevelRequestError> {
		self.requestx_api_client.get_gd_level_info(level_id).await
	}

	pub async fn level_request_service(
		&self,
		level_request: &LevelRequest,
	) -> Result<LevelRequest, LevelRequestError> {
		if !Self::is_valid_youtube_link(&level_request.youtube_video_link) {
			warn!("Invalid link: {}", &level_request.youtube_video_link);
			return Err(LevelRequestError::SerializeError(
				level_request.youtube_video_link.clone(),
			));
		}

		self.requestx_api_client
			.create_level_request(level_request)
			.await
	}

	pub async fn update_level_request(
		&self,
		level_request: UpdateLevelRequest,
	) -> Result<LevelRequest, LevelRequestError> {
		if let Some(youtube_video_link) = &level_request.youtube_video_link {
			if !Self::is_valid_youtube_link(youtube_video_link) {
				warn!("Invalid link: {}", &youtube_video_link);
				return Err(LevelRequestError::SerializeError(
					youtube_video_link.clone(),
				));
			}
		}

		self.requestx_api_client
			.update_level_request(level_request)
			.await
	}

	pub async fn delete_level_request(
		&self,
		level_id: u64,
	) -> Result<LevelRequest, LevelRequestError> {
		self.requestx_api_client
			.delete_level_request(level_id)
			.await
	}

	pub async fn update_request_message_id(
		&self,
		update_level_request_message: UpdateLevelRequestMessageId,
	) -> Result<(), LevelRequestError> {
		self.requestx_api_client
			.update_level_request_message_id(update_level_request_message)
			.await
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
