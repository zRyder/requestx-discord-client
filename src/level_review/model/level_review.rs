use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelReview {
	#[serde(rename = "reviewer_discord_id")]
	pub discord_user_id: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discord_message_id: Option<u64>,
	pub level_id: u64,
	pub review_contents: String,
}

impl LevelReview {
	pub fn new(
		discord_user_id: u64,
		discord_message_id: Option<u64>,
		level_id: u64,
		review_contents: String,
	) -> Self {
		Self {
			discord_user_id,
			discord_message_id,
			level_id,
			review_contents,
		}
	}
}

#[derive(Serialize)]
pub struct UpdateLevelReviewMessageId {
	#[serde(rename = "discord_id")]
	pub discord_user_id: u64,
	pub level_id: u64,
	pub discord_message_id: u64,
}

impl UpdateLevelReviewMessageId {
	pub fn new(discord_user_id: u64, level_id: u64, discord_message_id: u64) -> Self {
		Self {
			discord_user_id,
			level_id,
			discord_message_id,
		}
	}
}
