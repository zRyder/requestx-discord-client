use log::error;
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client, StatusCode
};

use crate::{
	config::{
		client_config::CLIENT_CONFIG,
		constants::{APPLICATION_JSON, CONTENT_TYPE},
		requestx_api_config::{RequestxApiConfig, REQUESTX_API_CONFIG}
	},
	level_request::model::{
		level_request::{LevelRequest, UpdateLevelRequest, UpdateLevelRequestMessageId},
		level_request_error::{ErrorMessage, LevelRequestError}
	},
	level_review::model::{
		level_review::{LevelReview, UpdateLevelReviewMessageId},
		level_review_error::LevelReviewError
	},
	request_manager::model::request_manager::UpdateRequestManager,
	requestx_api::auth::auth_service::JWT,
	reviewer::model::{reviewer::CreateReviewerRequest, reviewer_error::ReviewerError},
	send_level::model::{
		moderator::{SendLevelRequest, SentLevel},
		send_level_error::ModeratorError
	},
	user::model::{discord_user::User, discord_user_error::DiscordUserError}
};
use crate::user::model::discord_user::{UserGDAccountLink, UserGDAccountLinkRequest};
use crate::user::model::discord_user_error::GDAccountLinkError;

pub struct RequestXApiClient<'a> {
	requestx_api_config: &'a RequestxApiConfig,
	web_client: Client
}

impl<'a> RequestXApiClient<'a> {
	pub fn new() -> Self {
		let mut default_headers = HeaderMap::new();
		default_headers.insert(CONTENT_TYPE, HeaderValue::from_static(APPLICATION_JSON));
		RequestXApiClient {
			requestx_api_config: &*REQUESTX_API_CONFIG,
			web_client: Client::builder()
				.default_headers(default_headers)
				.build()
				.expect("Client::new")
		}
	}

	pub async fn get_user(&self, discord_id: u64) -> Result<Option<User>, DiscordUserError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.get(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.get_user,
				discord_id
			))
			.headers(headers)
			.send()
			.await
			.map_err(|get_user_error| {
				error!(
					"Error making API call for get user to RequestX API: {}",
					get_user_error
				);
				DiscordUserError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.eq(&StatusCode::NOT_FOUND) {
			Ok(None)
		} else if status_code.is_server_error() || status_code.is_client_error() {
			let error_message =
				serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
			error!(
				"Error response received from RequestX API: {}",
				error_message.message
			);

			Err(DiscordUserError::RequestXApiError(error_message))
		} else {
			let discord_user_data: User =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					DiscordUserError::RequestError
				})?;

			Ok(Some(discord_user_data))
		}
	}

	pub async fn get_level_request(
		&self,
		level_id: u64
	) -> Result<Option<LevelRequest>, LevelRequestError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.get(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.request_level,
				level_id
			))
			.headers(headers)
			.send()
			.await
			.map_err(|get_level_request_error| {
				error!(
					"Error making API call for level request to RequestX API: {}",
					get_level_request_error
				);
				LevelRequestError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.eq(&StatusCode::NOT_FOUND) {
			Ok(None)
		} else if status_code.is_server_error() || status_code.is_client_error() {
			let error_message =
				serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
			error!(
				"Error response received from RequestX API: {}",
				error_message.message
			);

			Err(LevelRequestError::RequestXApiError(error_message))
		} else {
			let level_request_data: LevelRequest =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					LevelRequestError::RequestError
				})?;

			Ok(Some(level_request_data))
		}
	}

	pub async fn get_level_review(
		&self,
		reviewer_discord_id: u64,
		level_id: u64
	) -> Result<Option<LevelReview>, LevelReviewError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.get(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.review_level,
				level_id
			))
			.query(&[("discord_id", reviewer_discord_id)])
			.headers(headers)
			.send()
			.await
			.map_err(|get_level_review_error| {
				error!(
					"Error making API call for level review to RequestX API: {}",
					get_level_review_error
				);
				LevelReviewError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.eq(&StatusCode::NOT_FOUND) {
			Ok(None)
		} else if status_code.is_server_error() || status_code.is_client_error() {
			let error_message =
				serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
			error!(
				"Error response received from RequestX API: {}",
				error_message.message
			);
			Err(LevelReviewError::RequestXApiError(error_message))
		} else {
			let level_review_data: LevelReview =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					LevelReviewError::RequestError
				})?;

			Ok(Some(level_review_data))
		}
	}

	pub async fn create_level_request(
		&self,
		level_request: &LevelRequest
	) -> Result<LevelRequest, LevelRequestError> {
		let serialized_level_request =
			serde_json::to_string(&level_request).map_err(|serialize_error| {
				error!(
					"Unable to serialize level request to json: {}",
					serialize_error
				);
				LevelRequestError::SerializeError(serialize_error.to_string())
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.post(format!(
				"{}{}",
				self.requestx_api_config.base_url, self.requestx_api_config.paths.request_level
			))
			.body(serialized_level_request)
			.headers(headers)
			.send()
			.await
			.map_err(|create_level_request_error| {
				error!(
					"Error making API call for post level request to RequestX API: {}",
					create_level_request_error
				);
				LevelRequestError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			Err(Self::handle_level_request_error(status_code, response_body))
		} else {
			let level_request_data: LevelRequest =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					LevelRequestError::RequestError
				})?;

			Ok(level_request_data)
		}
	}

	pub async fn update_level_request(
		&self,
		update_level_request: UpdateLevelRequest
	) -> Result<LevelRequest, LevelRequestError> {
		let serialized_update_level_request = serde_json::to_string(&update_level_request)
			.map_err(|serialize_error| {
				error!(
					"Unable to serialize level request to json: {}",
					serialize_error
				);
				LevelRequestError::SerializeError(serialize_error.to_string())
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.patch(format!(
				"{}{}",
				self.requestx_api_config.base_url, self.requestx_api_config.paths.request_level
			))
			.body(serialized_update_level_request)
			.headers(headers)
			.send()
			.await
			.map_err(|update_level_request_error| {
				error!(
					"Error making API call for update level request to RequestX API: {}",
					update_level_request_error
				);
				LevelRequestError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			Err(RequestXApiClient::handle_level_request_error(
				status_code,
				response_body
			))
		} else {
			let level_request_data: LevelRequest =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					LevelRequestError::RequestError
				})?;

			Ok(level_request_data)
		}
	}

	pub async fn delete_level_request(
		&self,
		level_id: u64
	) -> Result<LevelRequest, LevelRequestError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.delete(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.request_level,
				level_id
			))
			.headers(headers)
			.send()
			.await
			.map_err(|delete_level_request_error| {
				error!(
					"Error making API call for update level request to RequestX API: {}",
					delete_level_request_error
				);
				LevelRequestError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			Err(RequestXApiClient::handle_level_request_error(
				status_code,
				response_body
			))
		} else {
			let level_request_data: LevelRequest =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					LevelRequestError::RequestError
				})?;

			Ok(level_request_data)
		}
	}

	pub async fn create_level_review(
		&self,
		level_review: &LevelReview
	) -> Result<LevelReview, LevelReviewError> {
		let serialized_create_level_review =
			serde_json::to_string(&level_review).map_err(|serialize_error| {
				error!(
					"Unable to serialize level review to json: {}",
					serialize_error
				);
				LevelReviewError::SerializeError
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.post(format!(
				"{}{}",
				self.requestx_api_config.base_url, self.requestx_api_config.paths.review_level
			))
			.body(serialized_create_level_review)
			.headers(headers)
			.send()
			.await
			.map_err(|create_level_review_error| {
				error!(
					"Error making API call for create level review to RequestX API: {}",
					create_level_review_error
				);
				LevelReviewError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			let error_message =
				serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
			error!(
				"Error response received from RequestX API: {}",
				error_message.message
			);

			Err(LevelReviewError::RequestXApiError(error_message))
		} else {
			let level_review_data: LevelReview =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					LevelReviewError::RequestError
				})?;

			Ok(level_review_data)
		}
	}

	pub async fn create_reviewer(
		&self,
		create_reviewer_request: CreateReviewerRequest
	) -> Result<(), ReviewerError> {
		let serialized_create_reviewer =
			serde_json::to_string(&create_reviewer_request).map_err(|serialize_error| {
				error!(
					"Unable to serialize level review to json: {}",
					serialize_error
				);
				ReviewerError::SerializeError
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.post(format!(
				"{}{}",
				self.requestx_api_config.base_url, self.requestx_api_config.paths.reviewer
			))
			.body(serialized_create_reviewer)
			.headers(headers)
			.send()
			.await
			.map_err(|create_reviewer_error| {
				error!(
					"Error making API call for create reviewer to RequestX API: {}",
					create_reviewer_error
				);
				ReviewerError::RequestError
			})?;

		let status_code = response.status();
		if status_code.is_client_error() || status_code.is_server_error() {
			let response_body = response.text().await.unwrap();
			let error_message =
				serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
			error!(
				"Error response received from RequestX API: {}",
				error_message.message
			);

			Err(ReviewerError::RequestXApiError)
		} else {
			Ok(())
		}
	}

	pub async fn remove_reviewer(&self, reviewer_discord_id: u64) -> Result<(), ReviewerError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.delete(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.reviewer,
				reviewer_discord_id
			))
			.headers(headers)
			.send()
			.await
			.map_err(|remove_reviewer_error| {
				error!(
					"Error making API call for remove reviewer to RequestX API: {}",
					remove_reviewer_error
				);
				ReviewerError::RequestError
			})?;

		let status_code = response.status();
		if status_code.is_client_error() || status_code.is_server_error() {
			let response_body = response.text().await.unwrap();
			let error_message =
				serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
			error!(
				"Error response received from RequestX API: {}",
				error_message.message
			);

			Err(ReviewerError::RequestXApiError)
		} else {
			Ok(())
		}
	}

	pub async fn send_level(
		&self,
		send_level: SendLevelRequest
	) -> Result<SentLevel, ModeratorError> {
		let serialized_send_level =
			serde_json::to_string(&send_level).map_err(|serialize_error| {
				error!(
					"Unable to serialize send level to json: {}",
					serialize_error
				);
				ModeratorError::SerializeError
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.post(format!(
				"{}{}",
				self.requestx_api_config.base_url, self.requestx_api_config.paths.send_level,
			))
			.body(serialized_send_level)
			.headers(headers)
			.send()
			.await
			.map_err(|send_level_error| {
				error!(
					"Error making API call for send level to RequestX API: {}",
					send_level_error
				);
				ModeratorError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			Err(RequestXApiClient::handle_moderator_client_error(
				status_code,
				response_body
			))
		} else {
			let send_level_data: SentLevel =
				serde_json::from_str(&response_body).map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					ModeratorError::RequestError
				})?;

			Ok(send_level_data)
		}
	}

	pub async fn init_gd_account_link(
		&self,
		user_gd_account_link_request: UserGDAccountLinkRequest
	) -> Result<UserGDAccountLink, GDAccountLinkError> {
		let serialized_init_gd_account_link = serde_json::to_string(&user_gd_account_link_request)
			.map_err(|serialize_error| {
				error!("Unable to serialize init gd account link request: {}", serialize_error);
				GDAccountLinkError::SerializeError
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.post(format!(
				"{}{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.gd_account_link,
			))
			.body(serialized_init_gd_account_link)
			.headers(headers)
			.send()
			.await
			.map_err(|init_gd_account_link_error| {
				error!(
					"Error making call for init gd account link to RequestX API: {}",
					init_gd_account_link_error
				);
				GDAccountLinkError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();
		if status_code.is_client_error() || status_code.is_server_error() {
			Err(RequestXApiClient::handle_init_gd_account_link_client_error(
				status_code,
				response_body
			))
		} else {
			let user_gd_account_link: UserGDAccountLink = serde_json::from_str(&response_body)
				.map_err(|deserialize_error| {
					error!(
						"Unable to deserialize OK response from RequestX API: {}",
						deserialize_error
					);
					GDAccountLinkError::RequestError
				})?;
			Ok(user_gd_account_link)
		}
	}

	pub async fn verify_gd_account_link(
		&self,
		discord_id: u64
	) -> Result<(), GDAccountLinkError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.get(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.gd_account_link,
				discord_id
			))
			.headers(headers)
			.send()
			.await
			.map_err(|init_gd_account_link_error| {
				error!(
					"Error making call for verify gd account link to RequestX API: {}",
					init_gd_account_link_error
				);
				GDAccountLinkError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();
		if status_code.is_client_error() || status_code.is_server_error() {
			Err(RequestXApiClient::handle_init_gd_account_link_client_error(
				status_code,
				response_body
			))
		} else {
			Ok(())
		}
	}

	pub async fn update_request_manager(
		&self,
		update_request_manager: &UpdateRequestManager
	) -> Result<(), LevelRequestError> {
		let serialized_update_request_manager = serde_json::to_string(&update_request_manager)
			.map_err(|serialize_error| {
				error!(
					"Unable to serialize update_request_manager: {}",
					serialize_error
				);
				LevelRequestError::SerializeError(serialize_error.to_string())
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.patch(format!(
				"{}{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.update_request_manager
			))
			.body(serialized_update_request_manager)
			.headers(headers)
			.send()
			.await
			.map_err(|update_request_manager_error| {
				error!(
					"Error making API call for update request manager to RequestX API: {}",
					update_request_manager_error
				);
				LevelRequestError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			Err(RequestXApiClient::handle_level_request_error(
				status_code,
				response_body
			))
		} else {
			Ok(())
		}
	}

	pub async fn update_level_request_message_id(
		&self,
		update_level_request_message_id: UpdateLevelRequestMessageId
	) -> Result<(), LevelRequestError> {
		let serialized_update_level_request_message_id =
			serde_json::to_string(&update_level_request_message_id).map_err(|serialize_error| {
				error!(
					"Unable to serialize update_level_request_message_id to json: {}",
					serialize_error
				);
				LevelRequestError::SerializeError(serialize_error.to_string())
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.patch(format!(
				"{}{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.update_request_message_id
			))
			.body(serialized_update_level_request_message_id)
			.headers(headers)
			.send()
			.await
			.map_err(|update_level_request_message_id_error| {
				error!(
					"Error making API call for update level request message id to RequestX API: {}",
					update_level_request_message_id_error
				);
				LevelRequestError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			Err(RequestXApiClient::handle_level_request_error(
				status_code,
				response_body
			))
		} else {
			Ok(())
		}
	}

	pub async fn update_level_review_message_id(
		&self,
		update_level_review_message_id: UpdateLevelReviewMessageId
	) -> Result<(), LevelReviewError> {
		let serialized_update_level_review_message_id =
			serde_json::to_string(&update_level_review_message_id).map_err(|serialize_error| {
				error!(
					"Unable to serialize update_level_review_message_id to json: {}",
					serialize_error
				);
				LevelReviewError::SerializeError
			})?;

		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.patch(format!(
				"{}{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.update_review_message_id
			))
			.body(serialized_update_level_review_message_id)
			.headers(headers)
			.send()
			.await
			.map_err(|update_level_review_message_id_error| {
				error!(
					"Error making API call for update level review message id to RequestX API: {}",
					update_level_review_message_id_error
				);
				LevelRequestError::RequestError
			})?;

		let status_code = response.status();
		let response_body = response.text().await.unwrap();

		if status_code.is_client_error() || status_code.is_server_error() {
			let error_message =
				serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
			error!(
				"Error response received from RequestX API: {}",
				error_message.message
			);

			Err(LevelReviewError::RequestXApiError(error_message))
		} else {
			Ok(())
		}
	}

	async fn get_auth_header(headers: &mut HeaderMap) {
		let _ = &JWT
			.get_jwt()
			.await
			.map_err(|get_jwt_error| error!("Error getting auth headers: {}", get_jwt_error))
			.map(|jwt| {
				headers.insert(
					&*REQUESTX_API_CONFIG.headers.requestx_discord_app_id,
					HeaderValue::from(CLIENT_CONFIG.discord_app_id)
				);
				headers.insert(
					"authorization",
					HeaderValue::from_str(format!("Bearer {}", jwt).as_str()).unwrap()
				);
			});
	}

	fn handle_level_request_error(
		response_status: StatusCode,
		response_body: String
	) -> LevelRequestError {
		let error_message =
			serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
		error!(
			"Error response received from RequestX API: {}",
			error_message.message
		);

		match response_status {
			StatusCode::CONFLICT => LevelRequestError::LevelRequestExists,
			StatusCode::TOO_MANY_REQUESTS => LevelRequestError::UserOnCooldown(
				serde_json::from_str::<User>(&*response_body).unwrap_or_default()
			),
			StatusCode::SERVICE_UNAVAILABLE => LevelRequestError::RequestsDisabled,
			_ => LevelRequestError::RequestXApiError(error_message)
		}
	}

	fn handle_moderator_client_error(
		response_status: StatusCode,
		response_body: String
	) -> ModeratorError {
		let error_message =
			serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
		error!(
			"Error response received from RequestX API: {}",
			error_message.message
		);

		if response_status.eq(&StatusCode::NOT_FOUND) {
			ModeratorError::LevelRequestDoesNotExist
		} else {
			ModeratorError::RequestXApiError
		}
	}

	fn handle_init_gd_account_link_client_error(
		response_status: StatusCode,
		response_body: String
	) -> GDAccountLinkError {
		let error_message =
			serde_json::from_str::<ErrorMessage>(&*response_body).unwrap_or_default();
		error!(
			"Error response received from RequestX API: {}",
			error_message.message
		);

		if response_status.eq(&StatusCode::NOT_FOUND) {
			GDAccountLinkError::GDAccountDoesNotExist
		} else if response_status.eq(&StatusCode::CONFLICT) {
			GDAccountLinkError::DiscordAccountAlreadyLinked
		} else if response_status.eq(&StatusCode::UNAUTHORIZED) {
			GDAccountLinkError::InvalidGDAccountLinkToken
		} else if response_status.eq(&StatusCode::GONE) {
			GDAccountLinkError::GDAccountLinkExpired
		}
		else {
			GDAccountLinkError::RequestXApiError(error_message)
		}
	}
}

#[cfg(test)]
mod tests {
	use httpmock::MockServer;
	use tokio_test::{assert_err, assert_ok};

	use crate::{
		config::requestx_api_config::REQUESTX_API_CONFIG,
		level_request::model::level_request::LevelRequest,
		requestx_api::requestx_api_client::RequestXApiClient,
		send_level::model::request_score::RequestRating
	};

	async fn init_mock_server() -> MockServer {
		let url = url::Url::parse(&*REQUESTX_API_CONFIG.base_url).unwrap();
		let host = url.host_str().unwrap();
		let port = url.port().unwrap();
		MockServer::connect_async(&*format!("{}:{}", host, port)).await
	}

	#[tokio::test]
	async fn level_request_should_succeed() {
		let server = init_mock_server().await;
		let test_request = LevelRequest {
			discord_user_id: 164072941645070336,
			level_id: 97624039,
			request_rating: RequestRating::One,
			youtube_video_link: "Some".to_string(),
			has_requested_feedback: false,
			notify: false,
			discord_message_id: None,
			level_name: None,
			level_author: None,
			level_length: None
		};
		let mock = server.mock(|when, then| {
			when.path(&*REQUESTX_API_CONFIG.paths.request_level)
				.body(serde_json::to_string(&test_request).unwrap());
			then.status(201);
		});

		let test_client = RequestXApiClient::new();

		assert_ok!(test_client.create_level_request(&test_request).await);
	}

	#[tokio::test]
	async fn level_request_should_fail_with_internal_server_error() {
		let server = init_mock_server().await;
		let test_request = LevelRequest {
			discord_user_id: 164072941645070336,
			level_id: 97624039,
			request_rating: RequestRating::One,
			youtube_video_link: "SOME".to_string(),
			has_requested_feedback: false,
			notify: false,
			discord_message_id: None,
			level_name: None,
			level_author: None,
			level_length: None
		};
		let mock = server.mock(|when, then| {
			when.path(&*REQUESTX_API_CONFIG.paths.request_level)
				.body(serde_json::to_string(&test_request).unwrap());
			then.status(500);
		});

		let test_client = RequestXApiClient::new();

		assert_err!(test_client.create_level_request(&test_request).await);
	}
}
