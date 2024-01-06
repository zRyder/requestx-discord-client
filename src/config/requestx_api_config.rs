use std::{fs, process};

use lazy_static::lazy_static;
use serde::Deserialize;
use toml::de::Error;

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

pub fn read_app_config() -> Result<RequestxApiConfig, Error> {
	let toml_str = fs::read_to_string("Config.toml").expect("Failed to read Cargo.toml file");
	toml::from_str(&toml_str)
}

lazy_static! {
	pub static ref REQUESTX_API_CONFIG: &'static RequestxApiConfig =
		{ &APP_CONFIG.requestx_api_config };
}
