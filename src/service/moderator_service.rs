use log::error;
use serenity::{all::CommandInteraction, client::Context};

use crate::{
	model::{
		level_request::UpdateLevelRequestThreadId,
		moderator::Moderator,
		requestx_api::{
			level_request_data::LevelRequestData, moderator_data::ModeratorError,
			requestx_api_client::RequestXApiClient
		}
	},
	service::level_request_service::LevelRequestService,
	util::discord::create_thread
};

pub struct ModeratorService<'a> {
	requestx_api_client: RequestXApiClient<'a>
}

impl<'a> ModeratorService<'a> {
	pub fn new() -> Self {
		ModeratorService {
			requestx_api_client: RequestXApiClient::new()
		}
	}

	pub async fn send_level(
		&self,
		ctx: &Context,
		command: &CommandInteraction,
		send_level_request: Moderator
	) -> Result<LevelRequestData, ModeratorError> {
		match self
			.requestx_api_client
			.make_send_level_request(send_level_request)
			.await
		{
			Ok(mut level_request_data) => {
				let thread_id;

				if let None = level_request_data.discord_thread_id {
					if let Ok(thread) = create_thread(
						ctx,
						&command,
						level_request_data.discord_message_id.unwrap(),
						&level_request_data
					)
					.await
					{
						thread_id = thread;
						level_request_data.discord_thread_id = Some(thread_id);

						let level_request_service = LevelRequestService::new();
						let update_level_request_thread_id = UpdateLevelRequestThreadId {
							level_id: level_request_data.level_id,
							discord_thread_id: thread_id
						};

						if let Err(update_level_request_thread_id_error) = level_request_service
							.update_request_thread_id(update_level_request_thread_id)
							.await
						{
							error!(
								"Unable to update level request thread ID: {}",
								update_level_request_thread_id_error
							);
							return Err(ModeratorError::RequestError);
						}
					} else {
						return Err(ModeratorError::RequestError);
					}
				}
				Ok(level_request_data)
			}
			Err(send_level_error) => Err(send_level_error)
		}
	}
}
