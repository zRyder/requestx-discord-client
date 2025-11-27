use serde::{Deserialize, Serialize};

use crate::send_level::model::request_score::{LevelLength, RequestRating};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelRequest {
	pub level_id: u64,
	#[serde(rename = "discord_id")]
	pub discord_user_id: u64,
	pub request_rating: RequestRating,
	pub youtube_video_link: String,
	pub has_requested_feedback: bool,
	pub notify: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discord_message_id: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none", flatten)]
	pub gd_level_info: Option<GdLevelInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GdLevelInfo {
	pub level_name: String,
	pub level_author: String,
	pub level_length: LevelLength,
}

impl LevelRequest {
	pub fn new(
		level_id: u64,
		discord_user_id: u64,
		request_score: RequestRating,
		youtube_video_link: String,
		has_requested_feedback: bool,
		notify: bool,
	) -> Self {
		Self {
			level_id,
			discord_user_id,
			request_rating: request_score,
			youtube_video_link,
			has_requested_feedback,
			notify,
			discord_message_id: None,
			gd_level_info: None,
		}
	}
}

#[derive(Serialize, Debug)]
pub struct UpdateLevelRequest {
	#[serde(rename = "discord_id")]
	pub discord_user_id: u64,
	pub level_id: u64,
	#[serde(rename = "request_rating")]
	pub request_score: Option<RequestRating>,
	pub youtube_video_link: Option<String>,
	pub has_requested_feedback: Option<bool>,
	pub notify: Option<bool>,
}

impl UpdateLevelRequest {
	pub fn new(
		discord_user_id: u64,
		level_id: u64,
		request_score: Option<RequestRating>,
		youtube_video_link: Option<String>,
		has_requested_feedback: Option<bool>,
		notify: Option<bool>,
	) -> Self {
		Self {
			discord_user_id,
			level_id,
			request_score,
			youtube_video_link,
			has_requested_feedback,
			notify,
		}
	}
}

#[derive(Serialize)]
pub struct UpdateLevelRequestMessageId {
	pub level_id: u64,
	pub discord_message_id: u64,
}
