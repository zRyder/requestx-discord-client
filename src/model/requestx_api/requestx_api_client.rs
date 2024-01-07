use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client, StatusCode
};

use crate::{
	config::{
		constants::{APPLICATION_JSON, CONTENT_TYPE},
		requestx_api_config::{RequestxApiConfig, REQUESTX_API_CONFIG}
	},
	model::{error::level_request_error::LevelRequestError, level_request::LevelRequest}
};
use crate::model::requestx_api::level_request_data::LevelRequestData;

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

	pub async fn make_requestx_api_level_request(
		self,
		level_request: LevelRequest
	) -> Result<LevelRequestData, LevelRequestError> {
		match serde_json::to_string(&level_request) {
			Ok(serialized_request) => {
				let response = self
					.web_client
					.post(format!(
						"{}{}",
						self.requestx_api_config.base_url,
						self.requestx_api_config.paths.request_level
					))
					.body(serialized_request)
					.send()
					.await;

				match response {
					Ok(response) => {
						if response.status().is_client_error() {
							Err(RequestXApiClient::handle_client_error(response.status()))
						} else if response.status().is_server_error() {
							Err(LevelRequestError::RequestXApiError)
						} else {
							let response_string = response.text().await.unwrap();
							let level_data: LevelRequestData = serde_json::from_str(&response_string).unwrap();
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

	fn handle_client_error(response_status: StatusCode) -> LevelRequestError {
		if response_status.eq(&StatusCode::CONFLICT) {
			LevelRequestError::LevelRequestExists
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
			youtube_video_link: None
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
			youtube_video_link: None
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
