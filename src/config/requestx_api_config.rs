use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestxApiConfig {
	pub base_url: String,
	pub paths: RequestxApiConfigPaths,
	pub headers: RequestxApiHeaders
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestxApiConfigPaths {
	pub request_level: String,
	pub review_level: String,
	pub reviewer: String,
	pub auth: String,
	pub send_level: String,
	pub update_request_message_id: String,
	pub update_review_message_id: String,
	pub update_request_thread_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestxApiHeaders {
	pub requestx_discord_app_id: String
}

lazy_static! {
	pub static ref REQUESTX_API_CONFIG: &'static RequestxApiConfig =
		&APP_CONFIG.requestx_api_config;
}
