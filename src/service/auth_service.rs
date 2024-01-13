use chrono::{Duration, Utc};
use jsonwebtoken::{decode, errors::Error, DecodingKey, Validation};
use lazy_static::lazy_static;
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Response, StatusCode
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
	config::{
		auth_config::AUTH_CONFIG, client_config::CLIENT_CONFIG,
		requestx_api_config::REQUESTX_API_CONFIG
	},
	model::error::auth_error::AuthError
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	aud: u64,
	iat: usize,
	exp: usize
}

lazy_static! {
	pub static ref JWT: Mutex<Option<String>> = Mutex::new(None);
}

impl JWT {
	pub async fn get_jwt(&self) -> Result<String, AuthError> {
		let mut jwt_lock = JWT.lock().await;

		if jwt_lock
			.as_ref()
			.map_or(true, |token| Self::is_expired(token))
		{
			match Self::generate_token().await {
				Ok(jwt) => {
					*jwt_lock = Some(jwt.clone());
					Ok(jwt)
				}
				Err(auth_error) => Err(auth_error)
			}
		} else {
			Ok(jwt_lock.as_ref().unwrap().clone())
		}
	}

	fn is_expired(token: &str) -> bool {
		if let Ok(token_data) = decode::<Claims>(
			token,
			&DecodingKey::from_secret(&AUTH_CONFIG.secret_token.as_ref()),
			&Validation::default()
		) {
			return token_data.claims.exp
				< (Utc::now() - Duration::minutes(AUTH_CONFIG.token_buffer as i64)).timestamp()
					as usize;
		} else {
			true
		}
	}

	async fn generate_token() -> Result<String, AuthError> {
		let requestx_auth_client = reqwest::Client::new();
		let mut headers = HeaderMap::new();
		headers.insert(
			&*REQUESTX_API_CONFIG.headers.requestx_discord_app_id,
			HeaderValue::from(CLIENT_CONFIG.discord_app_id)
		);
		headers.insert(
			&*AUTH_CONFIG.auth_header_name,
			HeaderValue::from_static(&AUTH_CONFIG.access_token)
		);

		match requestx_auth_client
			.post(format!(
				"{}{}",
				&REQUESTX_API_CONFIG.base_url, &REQUESTX_API_CONFIG.paths.auth,
			))
			.headers(headers)
			.send()
			.await
		{
			Ok(resp) => {
				if resp.status().eq(&StatusCode::CREATED) {
					Ok(resp
						.headers()
						.get("authorization")
						.unwrap()
						.to_str()
						.unwrap()
						.to_string())
				} else {
					Err(AuthError::Unauthorized)
				}
			}
			Err(_err) => Err(AuthError::AuthenticationFailed)
		}
	}
}
