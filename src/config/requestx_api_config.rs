use lazy_static::lazy_static;
use serde::Deserialize;

use crate::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize)]
pub struct RequestxApiConfig {
	pub base_url: String,
	pub paths: RequestxApiConfigPaths
}

#[derive(Debug, Deserialize)]
pub struct RequestxApiConfigPaths {
	pub request_level: String
}

lazy_static! {
	pub static ref REQUESTX_API_CONFIG: &'static RequestxApiConfig =
		&APP_CONFIG.requestx_api_config;
}
