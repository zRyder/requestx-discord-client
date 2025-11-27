use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct User {
	pub last_request_time: Option<DateTime<Utc>>,
	pub request_cooldown: u64,
}

impl User {
	pub fn format_cooldown(&self) -> Option<String> {
		let duration_since_last_request = if let Some(last_request_time) = self.last_request_time {
			(last_request_time + Duration::minutes(self.request_cooldown as i64)) - Utc::now()
		} else {
			return None;
		};

		let days = duration_since_last_request.num_days();
		let hours = duration_since_last_request.num_hours() - (days * 24);
		let minutes = duration_since_last_request.num_minutes() - (days * 1440 + hours * 60);
		let seconds = duration_since_last_request.num_seconds()
			- (days * 86400 + hours * 3600 + minutes * 60);

		// Display the duration
		let mut units = Vec::new();
		if days > 0 {
			units.push(format!("{} days", days));
		}
		if hours > 0 {
			units.push(format!("{} hours", hours));
		}
		if minutes > 0 {
			units.push(format!("{} minutes", minutes));
		}
		if seconds > 0 {
			units.push(format!("{} seconds", seconds));
		}

		let duration_str: Option<String>;

		if units.is_empty() {
			duration_str = None;
		} else {
			duration_str = if units.len() > 1 {
				let last_unit = units.pop().unwrap();
				let rest = units.join(", ");
				Some(format!("{} and {}", rest, last_unit))
			} else {
				Some(units.join(", "))
			};
		}
		duration_str
	}
}

#[derive(Serialize, Debug)]
pub struct UserGDAccountLinkRequest {
	pub discord_id: u64,
	pub gd_username: String,
}

impl UserGDAccountLinkRequest {
	pub fn new(discord_id: u64, gd_username: String) -> Self {
		Self {
			discord_id,
			gd_username,
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct UserGDAccountLink {
	pub discord_id: u64,
	pub gd_username: String,
	pub gd_player_id: u64,
	pub gd_account_requestx_token: String,
}
