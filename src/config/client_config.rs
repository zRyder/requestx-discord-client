use lazy_static::lazy_static;
use serde::Deserialize;

use crate::config::{
	common_config::APP_CONFIG,
	requestx_api_config::{RequestxApiConfigPaths, RequestxApiHeaders}
};

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
	pub discord_app_id: u64
}

lazy_static! {
	pub static ref CLIENT_CONFIG: &'static ClientConfig = &APP_CONFIG.client_config;
}
