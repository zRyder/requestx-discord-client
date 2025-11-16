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
			DiscordUserError::RequestXApiError(_) => {
				write!(f, "The server failed to get the user")
			}
		}
	}
}

impl Error for DiscordUserError {}

#[derive(Debug, PartialEq)]
pub enum GDAccountLinkError {
	GDAccountDoesNotExist,
	GDAccountLinkExpired,
	InvalidGDAccountLinkToken,
	DiscordAccountAlreadyLinked,
	SerializeError,
	RequestError,
	RequestXApiError(ErrorMessage)
}

impl Display for GDAccountLinkError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			GDAccountLinkError::GDAccountDoesNotExist => write!(f, "GD Account or user does not exist"),
			GDAccountLinkError::GDAccountLinkExpired => write!(f, "Token has expired"),
			GDAccountLinkError::InvalidGDAccountLinkToken => write!(f, "Invalid Token provided"),
			GDAccountLinkError::DiscordAccountAlreadyLinked => write!(f, "User has already been linked to a GD account"),
			GDAccountLinkError::SerializeError => write!(f, "Unable to serialize request"),
			GDAccountLinkError::RequestError => write!(f, "Unable to make request to server"),
			GDAccountLinkError::RequestXApiError(_) => write!(f, "The server returned an unknown error")
		}
	}
}

impl Error for GDAccountLinkError {}