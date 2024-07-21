use serde::Serialize;

#[derive(Serialize)]
pub struct GetDiscordUserRequest {
	pub discord_user_id: u64
}
