use std::{
	error::Error,
	fmt::{Display, Formatter}
};

#[derive(Debug, PartialEq)]
pub enum LevelReviewError {
	LevelRequestDoesNotExists,
	RequestError,
	SerializeError,
	RequestXApiError
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
			LevelReviewError::RequestXApiError => {
				write!(f, "The server failed to make the upload the level review")
			}
		}
	}
}

impl Error for LevelReviewError {}
