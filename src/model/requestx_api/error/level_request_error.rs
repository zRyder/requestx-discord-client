use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LevelRequestError {
	LevelRequestExists,
	RequestError,
	SerializeError,
	UserOnCooldown(UserOnCooldownError),
	RequestsDisabled,
	RequestXApiError
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserOnCooldownError {
	pub last_request_time: DateTime<Utc>,
	pub request_cooldown: u64
}

impl Display for LevelRequestError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelRequestError::LevelRequestExists => {
				write!(f, "Level has already been requested")
			}
			LevelRequestError::RequestError => {
				write!(f, "Unable to make request to server")
			}
			LevelRequestError::SerializeError => {
				write!(f, "Unable to serialize level request")
			}
			LevelRequestError::UserOnCooldown(cooldown_error_data) => {
				let duration = (cooldown_error_data.last_request_time
					+ Duration::minutes(cooldown_error_data.request_cooldown as i64))
					- Utc::now();
				write!(
					f,
					"You are still on cooldown you, you can request again in **{}**.",
					{
						let hours = duration.num_hours();
						let minutes = duration.num_minutes() - (hours * 60);
						let seconds = duration.num_seconds() - (hours * 3600 + minutes * 60);

						// Display the duration
						let mut units = Vec::new();
						if hours > 0 {
							units.push(format!("{} hours", hours));
						}
						if minutes > 0 {
							units.push(format!("{} minutes", minutes));
						}
						if seconds > 0 {
							units.push(format!("{} seconds", seconds));
						}

						let duration_str: String;

						if units.is_empty() {
							duration_str = "0 seconds".to_string()
						} else {
							duration_str = if units.len() > 1 {
								let last_unit = units.pop().unwrap();
								let rest = units.join(", ");
								format!("{} and {}", rest, last_unit)
							} else {
								units.join(", ")
							};
						}
						duration_str
					}
				)
			}
			LevelRequestError::RequestsDisabled => {
				write!(f, "Requests are currently disabled ")
			}
			LevelRequestError::RequestXApiError => {
				write!(f, "The server failed to make the level request")
			}
		}
	}
}

impl Error for LevelRequestError {}
