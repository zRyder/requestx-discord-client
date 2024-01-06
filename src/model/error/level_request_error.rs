use std::{
	error::Error,
	fmt::{Display, Formatter}
};

#[derive(Debug, PartialEq)]
pub enum LevelRequestError {
	LevelRequestExists,
	MalformedRequestError,
	RequestError,
	SerializeError,
	RequestXApiError
}

impl Display for LevelRequestError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			LevelRequestError::LevelRequestExists => {
				write!(f, "Level has already been requested")
			}
			LevelRequestError::MalformedRequestError => {
				write!(f, "The requested level was malformed")
			}
			LevelRequestError::RequestError => {
				write!(f, "Unable to make request to server")
			}
			LevelRequestError::SerializeError => {
				write!(f, "Unable to serialize level request")
			}
			LevelRequestError::RequestXApiError => {
				write!(f, "The server failed to make the level request")
			}
		}
	}
}

impl Error for LevelRequestError {}
