use serde::Serialize;

use crate::model::request_score::RequestRating;

#[derive(Serialize)]
pub struct LevelRequest {
	pub discord_user_id: u64,
	pub level_id: u64,
	pub request_score: RequestRating,
	pub video_link: Option<String>
}
