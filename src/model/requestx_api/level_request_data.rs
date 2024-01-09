use serde::{Deserialize, Serialize};

use crate::model::request_score::RequestRating;

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelRequestData {
	pub level_id: u64,
	pub discord_id: u64,
	pub discord_message_id: Option<u64>,
	pub discord_thread_id: Option<u64>,
	pub level_name: String,
	pub level_author: String,
	pub request_score: RequestRating,
	pub youtube_video_link: String
}
