use std::{fs, process};

use lazy_static::lazy_static;
use serde::Deserialize;
use toml::de::Error;

use crate::config::requestx_api_config::RequestxApiConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub requestx_api_config: RequestxApiConfig
}

pub fn init_app_config() -> Result<AppConfig, Error> { read_app_config() }

fn read_app_config() -> Result<AppConfig, Error> {
	let path = if cfg!(test) {
		"Config_test.toml"
	} else {
		"Config.toml"
	};
	let toml_str = fs::read_to_string(path).expect("Failed to read Cargo.toml file");
	toml::from_str(&toml_str)
}

lazy_static! {
	pub static ref APP_CONFIG: AppConfig = {
		match read_app_config() {
			Ok(common_config) => common_config,
			Err(err) => {
				println!("{}", err);
				process::exit(1)
			}
		}
	};
}
