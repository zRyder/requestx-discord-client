use std::{
	error::Error,
	fmt::{Display, Formatter}
};

#[derive(Debug, PartialEq)]
pub enum AuthError {
	Unauthorized,
	AuthenticationFailed
}

impl Display for AuthError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			AuthError::Unauthorized => {
				write!(f, "Unauthorized")
			}
			AuthError::AuthenticationFailed => {
				write!(f, "There was an error while attempting to auhtenticate")
			}
		}
	}
}

impl Error for AuthError {}
