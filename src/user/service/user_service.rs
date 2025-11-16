use log::warn;

use crate::{
	requestx_api::requestx_api_client::RequestXApiClient,
	user::model::{discord_user::User, discord_user_error::DiscordUserError}
};
use crate::user::model::discord_user::{UserGDAccountLink, UserGDAccountLinkRequest};
use crate::user::model::discord_user_error::GDAccountLinkError;

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

	pub async fn init_gd_account_link(
		&self,
		user_gd_account_link_request: UserGDAccountLinkRequest
	) -> Result<UserGDAccountLink, GDAccountLinkError> {
		self.requestx_api_client
			.init_gd_account_link(user_gd_account_link_request)
			.await
			.map_err(GDAccountLinkError::from)
	}
	
	pub async fn verify_gd_account_link(
		&self,
		discord_user_id: u64,
	) -> Result<(), GDAccountLinkError> {
		self.requestx_api_client
			.verify_gd_account_link(discord_user_id)
			.await
			.map_err(GDAccountLinkError::from)
	}
}
