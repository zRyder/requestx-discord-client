use serde::Serialize;

#[derive(Serialize)]
pub struct AddReviewerRequest {
	pub reviewer_discord_id: u64
}

#[derive(Serialize)]
pub struct GetReviewerRequest {
	pub reviewer_discord_id: u64,
	pub is_active: bool
}

#[derive(Serialize)]
pub struct RemoveReviewerRequest {
	pub reviewer_discord_id: u64
}
