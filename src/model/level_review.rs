use serde::Serialize;

#[derive(Serialize)]
pub struct LevelReview {
	#[serde(rename = "reviewer_discord_id")]
	pub discord_user_id: u64,
	pub discord_message_id: Option<u64>,
	pub level_id: u64,
	pub review_contents: String
}

#[derive(Serialize)]
pub struct UpdateLevelReviewMessageId {
	#[serde(rename = "discord_id")]
	pub discord_user_id: u64,
	pub level_id: u64,
	pub discord_message_id: u64
}
