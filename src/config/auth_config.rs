use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
	pub secret_token: String,
	pub access_token: String,
	pub auth_header_name: String,
	pub token_buffer: i8
}

lazy_static! {
	pub static ref AUTH_CONFIG: &'static AuthConfig = &APP_CONFIG.auth_config;
}
