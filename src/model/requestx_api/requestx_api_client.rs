use reqwest::{header::{HeaderMap, HeaderValue}, Client, StatusCode, Response, Error};

use crate::{
	config::{
		constants::{APPLICATION_JSON, CONTENT_TYPE},
		requestx_api_config::{RequestxApiConfig, REQUESTX_API_CONFIG}
	},
	model::{
		error::level_request_error::LevelRequestError,
		level_request::{
			GetLevelRequest, GetLevelReview, LevelRequest, UpdateLevelRequestMessageId,
			UpdateLevelRequestThreadId
		},
		level_review::{LevelReview, UpdateLevelReviewMessageId},
		requestx_api::{
			level_request_data::LevelRequestData,
			level_review_data::LevelReviewData,
			level_review_error::LevelReviewError,
			reviewer_data::{ReviewerData, ReviewerError}
		},
		reviewer::{AddReviewerRequest, GetReviewerRequest, RemoveReviewerRequest}
	}
};
use crate::config::client_config::CLIENT_CONFIG;
use crate::model::moderator::Moderator;
use crate::model::requestx_api::moderator_data::ModeratorError;
use crate::service::auth_service::JWT;

pub struct RequestXApiClient<'a> {
	requestx_api_config: &'a RequestxApiConfig,
	web_client: Client
}

impl RequestXApiClient<'_> {
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

	pub async fn get_level_request(
		&self,
		get_level_request: GetLevelRequest
	) -> Result<Option<LevelRequestData>, LevelRequestError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.get(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.request_level,
				get_level_request.level_id
			))
			.headers(headers)
			.send()
			.await;

		match response {
			Ok(response) => {
				if response.status().eq(&StatusCode::NOT_FOUND) {
					Ok(None)
				} else if response.status().is_client_error() {
					Err(RequestXApiClient::handle_level_request_client_error(
						response.status()
					))
				} else if response.status().is_server_error() {
					Err(LevelRequestError::RequestXApiError)
				} else {
					let response_string = response.text().await.unwrap();
					let level_data: LevelRequestData =
						serde_json::from_str(&response_string).unwrap();
					Ok(Some(level_data))
				}
			}
			Err(error) => {
				println!("{}", error);
				Err(LevelRequestError::RequestError)
			}
		}
	}

	pub async fn get_level_review(
		&self,
		get_level_review: GetLevelReview
	) -> Result<Option<LevelReviewData>, LevelReviewError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.get(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.review_level,
				get_level_review.level_id
			))
			.query(&[("discord_id", get_level_review.discord_user_id)])
			.headers(headers)
			.send()
			.await;

		match response {
			Ok(response) => {
				if response.status().eq(&StatusCode::NOT_FOUND) {
					Ok(None)
				} else if response.status().is_client_error() {
					Err(RequestXApiClient::handle_level_review_client_error(
						response.status()
					))
				} else if response.status().is_server_error() {
					Err(LevelReviewError::RequestXApiError)
				} else {
					let response_string = response.text().await.unwrap();
					let level_review_data: LevelReviewData =
						serde_json::from_str(&response_string).unwrap();
					Ok(Some(level_review_data))
				}
			}
			Err(error) => {
				println!("{}", error);
				Err(LevelReviewError::RequestError)
			}
		}
	}

	pub async fn make_requestx_api_level_request(
		&self,
		level_request: LevelRequest
	) -> Result<LevelRequestData, LevelRequestError> {
		match serde_json::to_string(&level_request) {
			Ok(serialized_request) => {
				let mut headers = HeaderMap::new();
				Self::get_auth_header(&mut headers).await;
				let response = self
					.web_client
					.post(format!(
						"{}{}",
						self.requestx_api_config.base_url,
						self.requestx_api_config.paths.request_level
					))
					.body(serialized_request)
					.headers(headers)
					.send()
					.await;

				match response {
					Ok(response) => {
						if response.status().is_client_error() {
							Err(RequestXApiClient::handle_level_request_client_error(
								response.status()
							))
						} else if response.status().is_server_error() {
							Err(LevelRequestError::RequestXApiError)
						} else {
							let response_string = response.text().await.unwrap();
							let level_data: LevelRequestData =
								serde_json::from_str(&response_string).unwrap();
							println!("{:?}", level_data);
							Ok(level_data)
						}
					}
					Err(error) => {
						println!("{}", error);
						Err(LevelRequestError::RequestError)
					}
				}
			}
			Err(err) => {
				// fail
				Err(LevelRequestError::SerializeError)
			}
		}
	}

	pub async fn make_requestx_api_level_review_request(
		&self,
		level_review: &LevelReview
	) -> Result<LevelReviewData, LevelReviewError> {
		match serde_json::to_string(&level_review) {
			Ok(serialized_request) => {
				let mut headers = HeaderMap::new();
				Self::get_auth_header(&mut headers).await;
				let response = self
					.web_client
					.post(format!(
						"{}{}",
						self.requestx_api_config.base_url,
						self.requestx_api_config.paths.review_level
					))
					.body(serialized_request)
					.headers(headers)
					.send()
					.await;

				match response {
					Ok(response) => {
						if response.status().is_client_error() {
							Err(RequestXApiClient::handle_level_review_client_error(
								response.status()
							))
						} else if response.status().is_server_error() {
							Err(LevelReviewError::RequestXApiError)
						} else {
							let response_string = response.text().await.unwrap();
							let level_review_data: LevelReviewData =
								serde_json::from_str(&response_string).unwrap();
							Ok(level_review_data)
						}
					}
					Err(error) => {
						println!("{}", error);
						Err(LevelReviewError::RequestError)
					}
				}
			}
			Err(err) => Err(LevelReviewError::SerializeError)
		}
	}

	pub async fn make_get_reviewer_request(
		&self,
		get_reviewer_request: GetReviewerRequest
	) -> Result<Option<ReviewerData>, ReviewerError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.get(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.reviewer,
				get_reviewer_request.reviewer_discord_id
			))
			.query(&[("is_active", get_reviewer_request.is_active)])
			.headers(headers)
			.send()
			.await;

		match response {
			Ok(response) => {
				if response.status().eq(&StatusCode::NOT_FOUND) {
					Ok(None)
				} else if response.status().is_client_error() {
					Err(RequestXApiClient::handle_reviewer_client_error(
						response.status()
					))
				} else if response.status().is_server_error() {
					Err(ReviewerError::RequestXApiError)
				} else {
					let response_string = response.text().await.unwrap();
					let reviewer_data: ReviewerData =
						serde_json::from_str(&response_string).unwrap();
					Ok(Some(reviewer_data))
				}
			}
			Err(error) => {
				println!("{}", error);
				Err(ReviewerError::RequestError)
			}
		}
	}

	pub async fn make_add_reviewer_request(
		&self,
		create_reviewer_request: AddReviewerRequest
	) -> Result<(), ReviewerError> {
		match serde_json::to_string(&create_reviewer_request) {
			Ok(serialized_request) => {
				let mut headers = HeaderMap::new();
				Self::get_auth_header(&mut headers).await;
				let response = self
					.web_client
					.post(format!(
						"{}{}",
						self.requestx_api_config.base_url, self.requestx_api_config.paths.reviewer
					))
					.body(serialized_request)
					.headers(headers)
					.send()
					.await;

				match response {
					Ok(response) => {
						if response.status().is_client_error() {
							Err(RequestXApiClient::handle_reviewer_client_error(
								response.status()
							))
						} else if response.status().is_server_error() {
							Err(ReviewerError::RequestXApiError)
						} else {
							Ok(())
						}
					}
					Err(error) => {
						println!("{}", error);
						Err(ReviewerError::RequestError)
					}
				}
			}
			Err(err) => {
				// fail
				Err(ReviewerError::SerializeError)
			}
		}
	}

	pub async fn make_remove_reviewer_request(
		&self,
		remove_reviewer_request: RemoveReviewerRequest
	) -> Result<(), ReviewerError> {
		let mut headers = HeaderMap::new();
		Self::get_auth_header(&mut headers).await;
		let response = self
			.web_client
			.delete(format!(
				"{}{}/{}",
				self.requestx_api_config.base_url,
				self.requestx_api_config.paths.reviewer,
				remove_reviewer_request.reviewer_discord_id
			))
			.headers(headers)
			.send()
			.await;

		match response {
			Ok(response) => {
				if response.status().is_client_error() {
					Err(RequestXApiClient::handle_reviewer_client_error(
						response.status()
					))
				} else if response.status().is_server_error() {
					Err(ReviewerError::RequestXApiError)
				} else {
					Ok(())
				}
			}
			Err(error) => {
				println!("{}", error);
				Err(ReviewerError::RequestError)
			}
		}
	}

	pub async fn make_send_level_request(
		&self,
		send_level_request: Moderator
	) -> Result<LevelRequestData, ModeratorError> {
		match serde_json::to_string(&send_level_request) {
			Ok(serialized_request) => {
				let mut headers = HeaderMap::new();
				Self::get_auth_header(&mut headers).await;
				let response = self
					.web_client
					.post(format!(
						"{}{}",
						self.requestx_api_config.base_url,
						self.requestx_api_config.paths.send_level,
					))
					.body(serialized_request)
					.headers(headers)
					.send()
					.await;

				match response {
					Ok(resp) => {
						if resp.status().is_client_error() {
							Err(RequestXApiClient::handle_moderator_client_error(resp.status()))
						} else if resp.status().is_server_error() {
							Err(ModeratorError::RequestXApiError)
						} else {
							let response_string = resp.text().await.unwrap();
							let level_request_data: LevelRequestData =
								serde_json::from_str(&response_string).unwrap();
							Ok(level_request_data)
						}
					}
					Err(send_level_error) => {
						Err(ModeratorError::RequestError)
					}
				}
			}
			Err(serialize_error) => {
				Err(ModeratorError::SerializeError)
			}
		}
	}

	pub async fn update_review_message_id(
		&self,
		update_level_review: UpdateLevelReviewMessageId
	) -> Result<(), LevelReviewError> {
		match serde_json::to_string(&update_level_review) {
			Ok(serialized_request) => {
				let mut headers = HeaderMap::new();
				Self::get_auth_header(&mut headers).await;
				let response = self
					.web_client
					.patch(format!(
						"{}{}",
						self.requestx_api_config.base_url,
						self.requestx_api_config.paths.update_review_message_id
					))
					.body(serialized_request)
					.headers(headers)
					.send()
					.await;

				match response {
					Ok(response) => {
						if response.status().is_client_error() {
							Err(RequestXApiClient::handle_level_review_client_error(
								response.status()
							))
						} else if response.status().is_server_error() {
							Err(LevelReviewError::RequestXApiError)
						} else {
							Ok(())
						}
					}
					Err(error) => {
						println!("{}", error);
						Err(LevelReviewError::RequestError)
					}
				}
			}
			Err(err) => {
				// fail
				Err(LevelReviewError::SerializeError)
			}
		}
	}

	pub async fn update_request_message_id(
		&self,
		update_level_request: UpdateLevelRequestMessageId
	) -> Result<(), LevelRequestError> {
		match serde_json::to_string(&update_level_request) {
			Ok(serialized_request) => {
				let mut headers = HeaderMap::new();
				Self::get_auth_header(&mut headers).await;
				let response = self
					.web_client
					.patch(format!(
						"{}{}",
						self.requestx_api_config.base_url,
						self.requestx_api_config.paths.update_request_message_id
					))
					.body(serialized_request)
					.headers(headers)
					.send()
					.await;

				match response {
					Ok(response) => {
						if response.status().is_client_error() {
							Err(RequestXApiClient::handle_level_request_client_error(
								response.status()
							))
						} else if response.status().is_server_error() {
							Err(LevelRequestError::RequestXApiError)
						} else {
							Ok(())
						}
					}
					Err(error) => {
						println!("{}", error);
						Err(LevelRequestError::RequestError)
					}
				}
			}
			Err(err) => {
				// fail
				Err(LevelRequestError::SerializeError)
			}
		}
	}

	pub async fn update_request_thread_id(
		&self,
		update_level_request: UpdateLevelRequestThreadId
	) -> Result<(), LevelRequestError> {
		match serde_json::to_string(&update_level_request) {
			Ok(serialized_request) => {
				let mut headers = HeaderMap::new();
				Self::get_auth_header(&mut headers).await;
				let response = self
					.web_client
					.patch(format!(
						"{}{}",
						self.requestx_api_config.base_url,
						self.requestx_api_config.paths.update_request_thread_id
					))
					.body(serialized_request)
					.headers(headers)
					.send()
					.await;

				match response {
					Ok(response) => {
						if response.status().is_client_error() {
							Err(RequestXApiClient::handle_level_request_client_error(
								response.status()
							))
						} else if response.status().is_server_error() {
							Err(LevelRequestError::RequestXApiError)
						} else {
							Ok(())
						}
					}
					Err(error) => {
						println!("{}", error);
						Err(LevelRequestError::RequestError)
					}
				}
			}
			Err(err) => {
				// fail
				Err(LevelRequestError::SerializeError)
			}
		}
	}

	async fn get_auth_header(headers: &mut HeaderMap) {
		match &JWT.get_jwt().await {
			Ok(jwt) => {
				headers.insert(
					&*REQUESTX_API_CONFIG.headers.requestx_discord_app_id,
					HeaderValue::from(CLIENT_CONFIG.discord_app_id)
				);
				headers.insert(
					"authorization",
					HeaderValue::from_str(format!("Bearer {}", jwt).as_str()).unwrap()
				);
			}
			Err(error) => {
			}
		}
	}

	fn handle_level_request_client_error(response_status: StatusCode) -> LevelRequestError {
		if response_status.eq(&StatusCode::CONFLICT) {
			LevelRequestError::LevelRequestExists
		} else {
			LevelRequestError::RequestXApiError
		}
	}

	fn handle_level_review_client_error(response_status: StatusCode) -> LevelReviewError {
		LevelReviewError::RequestXApiError
	}

	fn handle_reviewer_client_error(response_status: StatusCode) -> ReviewerError {
		ReviewerError::RequestXApiError
	}

	fn handle_moderator_client_error(response_status: StatusCode) -> ModeratorError {
		if response_status.eq(&StatusCode::NOT_FOUND) {
			ModeratorError::LevelRequestDoesNotExist
		} else {
			ModeratorError::RequestXApiError
		}
	}

	fn handle_update_request_message_id_client_error(
		response_status: StatusCode
	) -> LevelRequestError {
		if response_status.eq(&StatusCode::NOT_FOUND) {
			LevelRequestError::LevelRequestDoesNotExists
		} else {
			LevelRequestError::RequestXApiError
		}
	}
}

#[cfg(test)]
mod tests {
	use httpmock::MockServer;
	use tokio_test::{assert_err, assert_ok};

	use crate::{
		config::requestx_api_config::REQUESTX_API_CONFIG,
		model::{
			level_request::LevelRequest, request_score::RequestRating,
			requestx_api::requestx_api_client::RequestXApiClient
		}
	};

	async fn init_mock_server() -> MockServer {
		println!("{}", &*REQUESTX_API_CONFIG.base_url);
		let url = url::Url::parse(&*REQUESTX_API_CONFIG.base_url).unwrap();
		let host = url.host_str().unwrap();
		let port = url.port().unwrap();
		MockServer::connect_async(&*format!("{}:{}", host, port)).await
	}

	#[tokio::test]
	async fn level_request_should_succeed() {
		let server = init_mock_server().await;
		println!("{}", server.base_url());
		let test_request = LevelRequest {
			discord_user_id: 164072941645070336,
			level_id: 97624039,
			request_score: RequestRating::One,
			youtube_video_link: "Some".to_string(),
			has_requested_feedback: false,
		};
		let mock = server.mock(|when, then| {
			when.path(&*REQUESTX_API_CONFIG.paths.request_level)
				.body(serde_json::to_string(&test_request).unwrap());
			then.status(201);
		});

		let test_client = RequestXApiClient::new();

		assert_ok!(
			test_client
				.make_requestx_api_level_request(test_request)
				.await
		);
	}

	#[tokio::test]
	async fn level_request_should_fail_with_internal_server_error() {
		let server = init_mock_server().await;
		let test_request = LevelRequest {
			discord_user_id: 164072941645070336,
			level_id: 97624039,
			request_score: RequestRating::One,
			youtube_video_link: "SOME".to_string(),
			has_requested_feedback: false,
		};
		let mock = server.mock(|when, then| {
			when.path(&*REQUESTX_API_CONFIG.paths.request_level)
				.body(serde_json::to_string(&test_request).unwrap());
			then.status(500);
		});

		let test_client = RequestXApiClient::new();

		assert_err!(
			test_client
				.make_requestx_api_level_request(test_request)
				.await
		);
	}
}
