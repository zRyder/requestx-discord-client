use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelReviewData {
	pub level_id: u64,
	#[serde(rename = "reviewer_discord_id")]
	pub discord_user_id: u64,
	pub discord_message_id: Option<u64>,
	pub review_contents: String
}
