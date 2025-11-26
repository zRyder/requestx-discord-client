use async_once_cell::OnceCell;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{debug, error, warn};
use reqwest::{
	header::{HeaderMap, HeaderValue},
	StatusCode
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
	config::{
		auth_config::AUTH_CONFIG, client_config::CLIENT_CONFIG, constants::CONTENT_LENGTH,
		requestx_api_config::REQUESTX_API_CONFIG
	},
	requestx_api::auth::auth_error::AuthError
};

pub struct RequestXAuthService{}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	aud: u64,
	iat: usize,
	exp: usize
}

static JWT: OnceCell<Mutex<String>> = OnceCell::new();

impl RequestXAuthService {

	pub async fn get_jwt() -> Result<String, AuthError> {
		let jwt_lock: &Mutex<String> = JWT.get_or_try_init(async {
			let token = Mutex::new(Self::generate_token().await?);
			Ok(token)
		}).await?;

		let mut current_token = jwt_lock.lock().await;

		if Self::is_expired(&*current_token) {
			warn!("JWT is expired or null, generating new token");
			match Self::generate_token().await {
				Ok(new_token) => {
					*current_token = new_token.clone();
					Ok(new_token)
				}
				Err(auth_error) => {
					error!("Authentication failed: {}", auth_error);
					Err(auth_error)
				}
			}
		} else {
			Ok(current_token.clone())
		}
	}

	fn is_expired(token: &str) -> bool {
		if let Ok(token_data) = decode::<Claims>(
			token,
			&DecodingKey::from_secret(&AUTH_CONFIG.secret_token.as_ref()),
			&Validation::default()
		) {
			token_data.claims.exp
				< (Utc::now() - Duration::minutes(AUTH_CONFIG.token_buffer as i64)).timestamp()
				as usize
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
		headers.insert(CONTENT_LENGTH, HeaderValue::from(0));
		debug!(
			"Calling RequestX API at {}",
			&REQUESTX_API_CONFIG.paths.auth
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
			Err(err) => {
				error!("Error calling RequestX API to authenticate: {}", err);
				Err(AuthError::AuthenticationFailed)
			}
		}
	}
}
