use log::warn;

use crate::{
	requestx_api::requestx_api_client::RequestXApiClient,
	user::model::{discord_user::User, discord_user_error::DiscordUserError}
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

	pub async fn get_discord_user(&self, discord_id: u64) -> Result<User, DiscordUserError> {
		self.requestx_api_client
			.get_user(discord_id)
			.await
			.map_err(DiscordUserError::from)?
			.map_or_else(
				|| {
					warn!("Discord user with id {} does not exist", discord_id);
					Err(DiscordUserError::UserDoesNotExist)
				},
				|discord_user_data| Ok(User::from(discord_user_data))
			)
	}
}
