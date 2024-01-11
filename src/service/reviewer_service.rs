use serenity::{
	all::{CommandInteraction, Context, GuildId, Member, User},
	builder::{Builder, EditMember}
};

use crate::model::{
	requestx_api::{
		requestx_api_client::RequestXApiClient,
		reviewer_data::{ReviewerData, ReviewerError}
	},
	reviewer::{AddReviewerRequest, GetReviewerRequest, RemoveReviewerRequest}
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

	pub async fn get_reviewer(
		&self,
		reviewer_discord_id: u64,
		is_active: bool
	) -> Result<Option<ReviewerData>, ReviewerError> {
		let get_reviewer_request = GetReviewerRequest {
			reviewer_discord_id,
			is_active
		};

		match self
			.requestx_api_client
			.make_get_reviewer_request(get_reviewer_request)
			.await
		{
			Ok(resp) => Ok(resp),
			Err(error) => Err(error)
		}
	}

	pub async fn create_reviewer(
		&self,
		ctx: &Context,
		discord_user: &User
	) -> Result<(()), ReviewerError> {
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
				member.guild_id = GuildId::from(1192954008013385839);
				match member.add_role(&ctx.http, 1192955803418775612).await {
					Ok(()) => Ok(()),
					Err(error) => Err(ReviewerError::RequestError)
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
				member.guild_id = GuildId::from(1192954008013385839);
				match member.remove_role(&ctx.http, 1192955803418775612).await {
					Ok(()) => Ok(()),
					Err(error) => Err(ReviewerError::RequestError)
				}
			}
			Err(error) => Err(error)
		}
	}
}
