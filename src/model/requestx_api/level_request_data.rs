use serde::{Deserialize, Serialize};

use crate::model::request_score::{LevelLength, RequestRating};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelRequestData {
	pub level_id: u64,
	pub discord_id: u64,
	pub discord_message_id: Option<u64>,
	pub discord_thread_id: Option<u64>,
	pub level_name: Option<String>,
	pub level_author: Option<String>,
	pub level_length: Option<LevelLength>,
	pub request_score: RequestRating,
	pub youtube_video_link: String,
	pub has_requested_feedback: bool,
	pub notify: bool
}
