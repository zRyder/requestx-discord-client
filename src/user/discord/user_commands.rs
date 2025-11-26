use serenity::all::{
	CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
	MessageBuilder, ResolvedOption, ResolvedValue
};

use crate::{
	serenity::discord::{invoke_command_ephemeral, log_to_discord},
	user::{
		model::{discord_user::User, discord_user_error::DiscordUserError},
		service::user_service::UserService
	}
};
use crate::config::client_config::CLIENT_CONFIG;
use crate::serenity::discord::log_error_to_discord;

pub fn register_view_cooldown<'a>() -> CreateCommand<'a> {
	CreateCommand::new("view-cooldown")
		.description("Views the current time remaining before you can make another request.")
}

pub async fn run_view_cooldown(ctx: &Context, command: &CommandInteraction) {
	let discord_user_id = command.user.id.get();
	let service = UserService::new();

	match service.get_discord_user(discord_user_id).await {
		Ok(discord_user) => output_user_cooldown(&ctx, &command, &discord_user).await,
		Err(get_discord_user_error) => {
			let mut log_message = MessageBuilder::new();
			log_message = log_message.push_bold(format!("{} ", command.user.name).as_str());
			log_message = log_message.push_line(format!(
				"({}) cause an error when viewing cooldown",
				command.user.id
			).as_str());
			log_message = log_message.push_codeblock(format!("{:?}", get_discord_user_error).as_str(), Some("rust"));
			log_to_discord(ctx.clone(), log_message.build()).await;

			invoke_command_ephemeral(&get_discord_user_error.to_string(), &ctx, &command).await;
		}
	}
}

pub fn register_view_user_cooldown<'a>() -> CreateCommand<'a> {
	CreateCommand::new("view-user-cooldown")
		.description(
			"Views the current time remaining before a specific user can make another request."
		)
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
	let discord_ephemeral_message: String;
	let actor_user_id = command.user.id.get();
	if actor_user_id != CLIENT_CONFIG.discord_bot_admin_id {
		discord_ephemeral_message = "Forbidden".to_string();
		return invoke_command_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
	}
	let discord_user_request_id: u64;

	let command_options = &command.data.options();
	if let Some(ResolvedOption {
		value: ResolvedValue::User(user, _),
		..
	}) = command_options.get(0)
	{
		discord_user_request_id = user.id.get();
	} else {
		discord_ephemeral_message = "Unable to resolve user".to_string();
		return invoke_command_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
	}

	let user_service = UserService::new();
	match user_service.get_discord_user(discord_user_request_id).await {
		Ok(discord_user) => output_user_cooldown(&ctx, &command, &discord_user).await,
		Err(get_discord_user_error) => {
			if matches!(get_discord_user_error, DiscordUserError::UserDoesNotExist) {
				return invoke_command_ephemeral("You can request a level now", &ctx, &command).await;
			}

			log_error_to_discord(
				&command.user,
				"viewing cooldown",
				&get_discord_user_error,
				None,
				&ctx
			).await;
			invoke_command_ephemeral(&get_discord_user_error.to_string(), &ctx, &command).await;
		}
	}
}

async fn output_user_cooldown(ctx: &Context, command: &CommandInteraction, discord_user: &User) {
	if let Some(cooldown_string) = discord_user.format_cooldown() {
		invoke_command_ephemeral(
			format!(
				"You are still on cooldown, you can request again in **{}**.",
				cooldown_string
			)
			.as_str(),
			&ctx,
			&command
		)
		.await;
	} else {
		invoke_command_ephemeral("You can request a level now", &ctx, &command).await;
	}
}
