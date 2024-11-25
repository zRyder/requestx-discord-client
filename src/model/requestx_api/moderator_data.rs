use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use serde::{Deserialize, Serialize};

use crate::model::{
	moderator::{SuggestedRating, SuggestedScore},
	requestx_api::level_request_data::LevelRequestData
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendLevelData {
	pub level_request: LevelRequestData,
	pub moderator_data: ModeratorData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModeratorData {
	pub suggested_score: SuggestedScore,
	pub suggested_rating: SuggestedRating
}

#[derive(Debug, PartialEq)]
pub enum ModeratorError {
	LevelRequestDoesNotExist,
	RequestXApiError,
	SerializeError,
	RequestError
}

impl Display for ModeratorError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ModeratorError::LevelRequestDoesNotExist => {
				write!(f, "Level Request does not exist")
			}
			ModeratorError::RequestXApiError => {
				write!(f, "The server failed to make the send level request")
			}
			ModeratorError::SerializeError => {
				write!(f, "Unable to serialized send level request")
			}
			ModeratorError::RequestError => {
				write!(f, "Unable to make request to server")
			}
		}
	}
}

impl Error for ModeratorError {}
