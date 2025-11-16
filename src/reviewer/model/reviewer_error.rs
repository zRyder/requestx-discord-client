use std::{
	error::Error,
	fmt::{Display, Formatter}
};

#[derive(Debug, PartialEq)]
pub enum ReviewerError {
	RequestError,
	SerializeError,
	RequestXApiError
}

impl Display for ReviewerError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
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

impl Error for ReviewerError {}
