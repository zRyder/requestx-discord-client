use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DiscordUser {
	pub last_request_time: Option<DateTime<Utc>>,
	pub request_cooldown: u64
}
