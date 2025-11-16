use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use serde::{Deserialize, Serialize};
use crate::user::model::discord_user::User;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LevelRequestError {
	LevelRequestExists,
	RequestError,
	SerializeError(String),
	UserOnCooldown(User),
	RequestsDisabled,
	RequestXApiError(ErrorMessage)
}

// #[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
// pub struct UserOnCooldownError {
// 	pub last_request_time: DateTime<Utc>,
// 	pub request_cooldown: u64
// }

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct ErrorMessage {
	pub message: String
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
			LevelRequestError::SerializeError(_field) => {
				write!(
					f,
					"Unable to serialize level request, double check your YouTube link"
				)
			}
			LevelRequestError::UserOnCooldown(user) => {
				write!(
					f,
					"You are still on cooldown, you can request again in **{}**.",
					user.format_cooldown()
					.unwrap()
				)
			}
			LevelRequestError::RequestsDisabled => {
				write!(f, "Requests are currently disabled ")
			}
			LevelRequestError::RequestXApiError(_error_message) => {
				write!(f, "The server failed to make the level request")
			}
		}
	}
}

impl Error for LevelRequestError {}
