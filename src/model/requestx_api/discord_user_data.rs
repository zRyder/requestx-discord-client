use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUserData {
	pub discord_user_id: u64,
	pub last_request_time: Option<DateTime<Utc>>,
	pub request_cooldown: u64
}
