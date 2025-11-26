use std::{collections::HashMap, env, fs};
use std::sync::OnceLock;
use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Serialize};

use crate::config::{
	auth_config::AuthConfig, client_config::ClientConfig, requestx_api_config::RequestxApiConfig
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
	pub client_config: ClientConfig,
	pub requestx_api_config: RequestxApiConfig,
	pub auth_config: AuthConfig
}

pub fn init_app_config() -> &'static AppConfig {
	APP_CONFIG.get_or_init(|| {
		read_app_config()
			.expect("Failed to read config file")
		})
}

fn read_app_config() -> Result<AppConfig, ConfigError> {
	let env_vars: HashMap<String, String> = env::vars().collect();
	let mut settings = Config::builder();

	let handlebars = handlebars::Handlebars::new();
	let template_string;
	if cfg!(test) {
		template_string =
			fs::read_to_string("Config_test.toml").expect("Unable to open configuration file");
	} else {
		template_string =
			fs::read_to_string("Config.toml").expect("Unable to open configuration file");
	};

	let rendered = handlebars
		.render_template(template_string.as_str(), &env_vars)
		.expect("Unable to render template");
	settings = settings.add_source(File::from_str(rendered.as_str(), FileFormat::Toml));
	settings.build()?.try_deserialize::<AppConfig>()
}

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
