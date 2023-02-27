use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignRequestError(String);

impl Display for SignRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SignRequestError {}

impl From<&str> for SignRequestError {
    fn from(message: &str) -> Self {
        SignRequestError(message.to_string())
    }
}
