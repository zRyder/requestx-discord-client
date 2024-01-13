use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ModeratorError{
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