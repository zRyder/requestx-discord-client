use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::config::common_config::APP_CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientConfig {
	pub discord_bot_token: String,
	pub discord_app_id: u64,
	pub discord_guild_id: u64,
	pub discord_reviewer_role_id: u64
}

lazy_static! {
	pub static ref CLIENT_CONFIG: &'static ClientConfig = &APP_CONFIG.client_config;
}
