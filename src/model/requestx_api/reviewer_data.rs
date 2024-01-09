use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewerData {
	pub reviewer_discord_id: u64,
	pub is_active: bool
}

#[derive(Debug, PartialEq)]
pub enum ReviewerError {
	ReviewerDoesNotExist,
	RequestError,
	SerializeError,
	RequestXApiError
}

impl Display for ReviewerError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ReviewerError::ReviewerDoesNotExist => {
				write!(f, "Reviewer does not exist")
			}
			ReviewerError::RequestError => {
				write!(f, "Unable to make request to server")
			}
			ReviewerError::SerializeError => {
				write!(f, "Unable to serialized reviewer")
			}
			ReviewerError::RequestXApiError => {
				write!(f, "The server failed to handle the request")
			}
		}
	}
}
