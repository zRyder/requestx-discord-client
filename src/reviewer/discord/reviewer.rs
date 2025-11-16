use log::error;
use serenity::all::{
	CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId,
	Member, MessageBuilder, ResolvedOption, ResolvedValue
};

use crate::{
	config::{app_config::APP_CONFIG, client_config::CLIENT_CONFIG},
	reviewer::service::reviewer_service::ReviewerService,
	serenity::discord::{invoke_ephemeral, log_action_to_discord, log_error_to_discord}
};

pub fn register_add_reviewer() -> CreateCommand {
	CreateCommand::new("add-reviewer")
		.description("Adds a new level reviewer.")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::User,
				"user",
				"The user to grant the role of reviewer."
			)
			.required(true)
		)
}

pub async fn run_add_reviewer(ctx: &Context, command: &CommandInteraction) {
	let reviewer_service = ReviewerService::new();

	let command_options = &command.data.options();
	let discord_server = GuildId::from(CLIENT_CONFIG.discord_guild_id);
	let discord_ephemeral_message: String;
	let mut log_message = MessageBuilder::new();
	let user_to_promote: Member;

	let actor_user_id = command.user.id.get();
	if actor_user_id != APP_CONFIG.client_config.discord_bot_admin_id {
		discord_ephemeral_message = "Forbidden".to_string();
		return invoke_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
	}

	if let Some(ResolvedOption {
		value: ResolvedValue::User(user, _),
		..
	}) = command_options.get(0)
	{
		user_to_promote = discord_server.member(&ctx.http, user.id).await.unwrap();
	} else {
		discord_ephemeral_message = "Unable to resolve user.".to_string();
		return invoke_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
	}

	match reviewer_service
		.create_reviewer(user_to_promote.user.id.get())
		.await
	{
		Err(create_reviewer_error) => {
			discord_ephemeral_message = "Unable to promote user to reviewer.".to_string();
			log_error_to_discord(
				&command.user,
				"promoting user to reviewer",
				&create_reviewer_error,
				None,
				&ctx
			)
			.await;
		}
		Ok(()) => {
			discord_ephemeral_message = "User has been promoted to reviewer".to_string();

			if let Err(add_reviewer_role_error) = user_to_promote
				.add_role(&ctx.http, CLIENT_CONFIG.discord_reviewer_role_id)
				.await
			{
				log_message.push_line("But there was an error assigning the role".to_string());
				error!(
					"Error assigning reviewer role to member: {}",
					add_reviewer_role_error
				)
			}

			log_action_to_discord(
				&command.user,
				"promoted user to reviewer",
				Some(&user_to_promote.user.id.get()),
				&ctx
			)
			.await;
		}
	}

	invoke_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
}

pub fn register_remove_reviewer() -> CreateCommand {
	CreateCommand::new("remove-reviewer")
		.description("Removes the current level reviewer.")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::User,
				"user",
				"The user to revoke the role of reviewer."
			)
			.required(true)
		)
}

pub async fn run_remove_reviewer(ctx: &Context, command: &CommandInteraction) {
	let reviewer_service = ReviewerService::new();

	let command_options = &command.data.options();
	let discord_server = GuildId::from(CLIENT_CONFIG.discord_guild_id);
	let discord_ephemeral_message: String;
	let mut log_message = MessageBuilder::new();
	let user_to_demote: Member;

	let actor_user_id = command.user.id.get();
	if actor_user_id != APP_CONFIG.client_config.discord_bot_admin_id {
		discord_ephemeral_message = "Forbidden".to_string();
		return invoke_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
	}

	if let Some(ResolvedOption {
		value: ResolvedValue::User(user, _),
		..
	}) = command_options.get(0)
	{
		user_to_demote = discord_server.member(&ctx.http, user.id).await.unwrap();
	} else {
		discord_ephemeral_message = "Unable to resolve user.".to_string();
		return invoke_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
	}

	match reviewer_service
		.remove_reviewer(user_to_demote.user.id.get())
		.await
	{
		Err(remove_reviewer_error) => {
			discord_ephemeral_message = "Unable to demote user from reviewer.".to_string();
			log_error_to_discord(
				&command.user,
				"demoting user from reviewer",
				&remove_reviewer_error,
				Some(&user_to_demote.user.id.get()),
				&ctx
			)
			.await;
		}
		Ok(()) => {
			discord_ephemeral_message = "User has been demoted from reviewer.".to_string();

			if let Err(remove_reviewer_role_error) = user_to_demote
				.remove_role(&ctx.http, CLIENT_CONFIG.discord_reviewer_role_id)
				.await
			{
				log_message.push_line("But there was an error removing the role".to_string());
				error!(
					"Error removing reviewer role from member: {}",
					remove_reviewer_role_error
				)
			}

			log_action_to_discord(
				&command.user,
				"demoted user from reviewer",
				Some(&user_to_demote.user.id.get()),
				&ctx
			)
			.await;
		}
	}

	invoke_ephemeral(&discord_ephemeral_message, &ctx, &command).await;
}
