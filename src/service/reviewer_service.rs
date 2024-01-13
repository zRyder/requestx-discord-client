use log::error;
use serenity::all::{Context, GuildId, Member, User};

use crate::{
	config::client_config::CLIENT_CONFIG,
	model::{
		requestx_api::{requestx_api_client::RequestXApiClient, reviewer_data::ReviewerError},
		reviewer::{AddReviewerRequest, RemoveReviewerRequest}
	}
};

pub struct ReviewerService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> ReviewerService<'a> {
	pub fn new() -> Self {
		ReviewerService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn create_reviewer(
		&self,
		ctx: &Context,
		discord_user: &User
	) -> Result<(), ReviewerError> {
		let add_reviewer_request = AddReviewerRequest {
			reviewer_discord_id: discord_user.id.get()
		};

		match self
			.requestx_api_client
			.make_add_reviewer_request(add_reviewer_request)
			.await
		{
			Ok(()) => {
				let mut member = Member::default();
				member.user = discord_user.clone();
				member.guild_id = GuildId::from(CLIENT_CONFIG.discord_guild_id);
				match member
					.add_role(&ctx.http, CLIENT_CONFIG.discord_reviewer_role_id)
					.await
				{
					Ok(()) => Ok(()),
					Err(error) => {
						error!("Unable to add reviewer: {}", error);
						Err(ReviewerError::RequestError)
					}
				}
			}
			Err(error) => Err(error)
		}
	}

	pub async fn remove_reviewer(
		&self,
		ctx: &Context,
		discord_user: &User
	) -> Result<(), ReviewerError> {
		let remove_reviewer_request = RemoveReviewerRequest {
			reviewer_discord_id: discord_user.id.get()
		};

		match self
			.requestx_api_client
			.make_remove_reviewer_request(remove_reviewer_request)
			.await
		{
			Ok(()) => {
				let mut member = Member::default();
				member.user = discord_user.clone();
				member.guild_id = GuildId::from(CLIENT_CONFIG.discord_guild_id);
				match member
					.remove_role(&ctx.http, CLIENT_CONFIG.discord_reviewer_role_id)
					.await
				{
					Ok(()) => Ok(()),
					Err(error) => {
						error!("Unable to remove reviewer: {}", error);
						Err(ReviewerError::RequestError)
					}
				}
			}
			Err(error) => Err(error)
		}
	}
}
