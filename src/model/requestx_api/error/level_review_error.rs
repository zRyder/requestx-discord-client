use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use crate::model::requestx_api::error::level_request_error::ErrorMessage;

#[derive(Debug, PartialEq)]
pub enum LevelReviewError<'a> {
	LevelRequestDoesNotExists,
	RequestError,
	DiscordFormattingError(usize, &'a str),
	SerializeError,
	RequestXApiError(ErrorMessage)
}

impl<'a> Display for LevelReviewError<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelReviewError::LevelRequestDoesNotExists => {
				write!(f, "Level Request does not exist")
			}
			LevelReviewError::RequestError => {
				write!(f, "Unable to make request to server")
			}
			LevelReviewError::DiscordFormattingError(index, paragraph) => {
				write!(
					f,
					"Unable to format level review at paragraph {}: {}",
					index, paragraph
				)
			}
			LevelReviewError::SerializeError => {
				write!(f, "Unable to serialize the level review")
			}
			LevelReviewError::RequestXApiError(_error_message) => {
				write!(f, "The server failed to upload the level review")
			}
		}
	}
}

impl<'a> Error for LevelReviewError<'a> {}
