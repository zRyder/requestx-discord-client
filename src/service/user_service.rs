use crate::model::{
	requestx_api::{
		discord_user_data::DiscordUserData, error::discord_user_error::DiscordUserError,
		requestx_api_client::RequestXApiClient
	},
	user::GetDiscordUserRequest
};

pub struct UserService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> UserService<'a> {
	pub fn new() -> Self {
		UserService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn get_discord_user(
		&self,
		get_discord_user_request: GetDiscordUserRequest
	) -> Result<Option<DiscordUserData>, DiscordUserError> {
		match self
			.requestx_api_client
			.get_user(get_discord_user_request)
			.await
		{
			Ok(response) => Ok(response),
			Err(error) => Err(error)
		}
	}

	pub async fn get_discord_user_admin(
		&self,
		get_discord_user_request: GetDiscordUserRequest
	) -> Result<DiscordUserData, DiscordUserError> {
		match self
			.requestx_api_client
			.get_user(get_discord_user_request)
			.await
		{
			Ok(Some(response)) => Ok(response),
			Ok(None) => Err(DiscordUserError::UserDoesNotExist),
			Err(error) => Err(error)
		}
	}
}
