use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use crate::level_request::model::level_request_error::{ErrorMessage, LevelRequestError};

#[derive(Debug, PartialEq)]
pub enum LevelReviewError {
	LevelRequestDoesNotExists,
	RequestError,
	DiscordFormattingError(usize, String),
	UserHasNotRequestedFeedback,
	SerializeError,
	RequestXApiError(ErrorMessage)
}

impl Display for LevelReviewError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelReviewError::LevelRequestDoesNotExists => {
				write!(f, "Level Request does not exist.")
			}
			LevelReviewError::RequestError => {
				write!(f, "Unable to make request to server.")
			}
			LevelReviewError::DiscordFormattingError(index, paragraph) => {
				write!(
					f,
					"Unable to format level review at paragraph {}: {}",
					index, paragraph
				)
			}
			LevelReviewError::UserHasNotRequestedFeedback => {
				write!(
					f,
					"The user has not requested feedback for this level request."
				)
			}
			LevelReviewError::SerializeError => {
				write!(f, "Unable to serialize the level review.")
			}
			LevelReviewError::RequestXApiError(_error_message) => {
				write!(f, "The server failed to upload the level review.")
			}
		}
	}
}

impl From<LevelRequestError> for LevelReviewError {
	fn from(value: LevelRequestError) -> Self {
		match value {
			LevelRequestError::RequestError => LevelReviewError::RequestError,
			LevelRequestError::SerializeError(_field) => LevelReviewError::RequestError,
			LevelRequestError::RequestXApiError(error_message) => {
				LevelReviewError::RequestXApiError(error_message)
			}
			LevelRequestError::UserOnCooldown(_) => LevelReviewError::RequestError,
			LevelRequestError::RequestsDisabled => LevelReviewError::RequestError,
			LevelRequestError::LevelRequestExists => LevelReviewError::RequestError
		}
	}
}

impl Error for LevelReviewError {}
