use serenity::all::{
	CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
	MessageBuilder, ResolvedOption, ResolvedValue
};

use crate::{
	config::common_config::APP_CONFIG,
	model::{requestx_api::error::format_cooldown, user::GetDiscordUserRequest},
	serenity::discord::{invoke_ephermal, log_to_discord},
	service::user_service::UserService
};

pub fn register_view_cooldown() -> CreateCommand {
	CreateCommand::new("view-cooldown")
		.description("Views the current time remaining before you can make another request.")
}

pub async fn run_view_cooldown(ctx: &Context, command: &CommandInteraction) {
	let discord_user_request = GetDiscordUserRequest {
		discord_user_id: command.user.id.get()
	};

	let service = UserService::new();

	match service.get_discord_user(discord_user_request).await {
		Ok(Some(discord_user)) => {
			if let Some(last_request_time) = discord_user.last_request_time {
				match format_cooldown(last_request_time, discord_user.request_cooldown as i64) {
					Some(duration_string) => {
						invoke_ephermal(
							format!(
								"You are still on cooldown you, you can request again in **{}**.",
								duration_string
							)
							.as_str(),
							&ctx,
							&command
						)
						.await;
					}
					None => {
						invoke_ephermal("You can request a level now", &ctx, &command).await;
					}
				}
			} else {
				invoke_ephermal("You can request a level now", &ctx, &command).await;
			}
		}
		Ok(None) => {
			invoke_ephermal("User not found.", &ctx, &command).await;
		}
		Err(get_discord_user_error) => {
			invoke_ephermal(&get_discord_user_error.to_string(), &ctx, &command).await;
			{
				let mut log_message = MessageBuilder::new();
				log_message.push_bold(format!("{} ", command.user.name));
				log_message.push_line(format!(
					"({}) cause an error when viewing cooldown",
					command.user.id
				));
				log_message.push_codeblock(format!("{:?}", get_discord_user_error), Some("rust"));
				log_to_discord(ctx.clone(), log_message.build()).await
			}
		}
	}
}

pub fn register_view_user_cooldown() -> CreateCommand {
	CreateCommand::new("view-user-cooldown")
		.description("Views the current time remaining before you can make another request.")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::User,
				"discord_user_id",
				"The user who's cooldown to view."
			)
			.required(true)
		)
}

pub async fn run_view_user_cooldown(ctx: &Context, command: &CommandInteraction) {
	let content: String;
	let actor_user_id = command.user.id.get();
	if actor_user_id != APP_CONFIG.client_config.discord_bot_admin_id {
		content = "Forbidden".to_string();
		invoke_ephermal(&content, &ctx, &command).await;
	} else {
		let reviewer = &command.data.options();
		if let Some(ResolvedOption {
			value: ResolvedValue::User(user, _),
			..
		}) = reviewer.get(0)
		{
			let discord_user_request = GetDiscordUserRequest {
				discord_user_id: user.id.get()
			};

			let service = UserService::new();

			match service.get_discord_user_admin(discord_user_request).await {
				Ok(discord_user) => {
					if let Some(last_request_time) = discord_user.last_request_time {
						match format_cooldown(
							last_request_time,
							discord_user.request_cooldown as i64
						) {
							Some(duration_string) => {
								invoke_ephermal(format!("You are still on cooldown you, you can request again in **{}**.", duration_string).as_str(), &ctx, &command).await;
							}
							None => {
								invoke_ephermal("You can request a level now", &ctx, &command)
									.await;
							}
						}
					} else {
						invoke_ephermal("You can request a level now", &ctx, &command).await;
					}
				}
				Err(get_discord_user_error) => {
					invoke_ephermal(&get_discord_user_error.to_string(), &ctx, &command).await;
					{
						let mut log_message = MessageBuilder::new();
						log_message.push_bold(format!("{} ", command.user.name));
						log_message.push_line(format!(
							"({}) cause an error when viewing cooldown",
							command.user.id
						));
						log_message
							.push_codeblock(format!("{:?}", get_discord_user_error), Some("rust"));
						log_to_discord(ctx.clone(), log_message.build()).await
					}
				}
			}
		} else {
			content = "Unable to view cooldown".to_string();
			invoke_ephermal(&content, &ctx, &command).await;
		}
	}
}
