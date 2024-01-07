use serde::Serialize;

use crate::model::request_score::RequestRating;

#[derive(Serialize)]
pub struct LevelRequest {
	#[serde(rename = "discord_id")]
	pub discord_user_id: u64,
	pub level_id: u64,
	#[serde(rename = "request_rating")]
	pub request_score: RequestRating,
	pub youtube_video_link: String
}
