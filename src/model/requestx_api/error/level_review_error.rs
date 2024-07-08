use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use crate::model::requestx_api::error::level_request_error::ErrorMessage;

#[derive(Debug, PartialEq)]
pub enum LevelReviewError {
	LevelRequestDoesNotExists,
	RequestError,
	SerializeError,
	RequestXApiError(ErrorMessage)
}

impl Display for LevelReviewError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelReviewError::LevelRequestDoesNotExists => {
				write!(f, "Level Request does not exist")
			}
			LevelReviewError::RequestError => {
				write!(f, "Unable to make request to server")
			}
			LevelReviewError::SerializeError => {
				write!(f, "Unable to serialized level review")
			}
			LevelReviewError::RequestXApiError(_error_message) => {
				write!(f, "The server failed to upload the level review")
			}
		}
	}
}

impl Error for LevelReviewError {}
