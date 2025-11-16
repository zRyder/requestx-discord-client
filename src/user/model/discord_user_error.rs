use std::{
	error::Error,
	fmt::{Display, Formatter}
};

use crate::level_request::model::level_request_error::ErrorMessage;

#[derive(Debug, PartialEq)]
pub enum DiscordUserError {
	UserDoesNotExist,
	RequestError,
	RequestXApiError(ErrorMessage)
}

impl Display for DiscordUserError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			DiscordUserError::UserDoesNotExist => {
				write!(f, "User does not exist")
			}
			DiscordUserError::RequestError => {
				write!(f, "Unable to make request to server")
			}
			DiscordUserError::RequestXApiError(_error_message) => {
				write!(f, "The server failed to get the user")
			}
		}
	}
}

impl Error for DiscordUserError {}
