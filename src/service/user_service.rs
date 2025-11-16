use log::warn;

use crate::model::{
	discord_user::DiscordUser, error::discord_user_error::DiscordUserError,
	requestx_api::requestx_api_client::RequestXApiClient
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

	pub async fn get_discord_user(&self, discord_id: u64) -> Result<DiscordUser, DiscordUserError> {
		self.requestx_api_client
			.get_user(discord_id)
			.await
			.map_err(DiscordUserError::from)?
			.map_or_else(
				|| {
					warn!("Discord user with id {} does not exist", discord_id);
					Err(DiscordUserError::UserDoesNotExist)
				},
				|discord_user_data| Ok(DiscordUser::from(discord_user_data))
			)
	}
}
