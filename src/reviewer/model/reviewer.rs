use serde::Serialize;

#[derive(Serialize)]
pub struct CreateReviewerRequest {
	pub reviewer_discord_id: u64
}
