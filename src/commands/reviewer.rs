use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, MessageBuilder, ResolvedOption, ResolvedValue};

use crate::{service::reviewer_service::ReviewerService, util::discord::invoke_ephermal};
use crate::util::discord::log_to_discord;

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
	let content: String;
	let actor_user_id = command.user.id.get();
	if actor_user_id != 164072941645070336 {
		content = "Forbidden".to_string();
		invoke_ephermal(&content, &ctx, &command).await;
	} else {
		let reviewer = &command.data.options();
		if let Some(ResolvedOption {
			value: ResolvedValue::User(user, _),
			..
		}) = reviewer.get(0)
		{
			let reviewer_service = ReviewerService::new();

			match reviewer_service.create_reviewer(&ctx, &user).await {
				Ok(()) => {
					content = "User has been promoted to reviewer".to_string();
					invoke_ephermal(&content, &ctx, &command).await;

					{
						let mut log_message = MessageBuilder::new();
						log_message.push_bold(format!("{} ", &user.name));
						log_message.push_line(format!("({}) has been promoted Reviewer", &user.id));
						log_to_discord(
							log_message.build(),
							ctx.clone(),
						).await
					}
				}
				Err(error) => {
					content = "Unable to add reviewer".to_string();
					invoke_ephermal(&content, &ctx, &command).await;

					{
						let mut log_message = MessageBuilder::new();
						log_message.push("Error promoting ".to_string());
						log_message.push_bold(format!("{} ", &user.name));
						log_message.push_line(format!("({}) to reviewer", &user.id));
						log_message.push_codeblock(format!("{:?}", error), Some("rust"));
						log_to_discord(
							log_message.build(),
							ctx.clone(),
						).await
					}
				}
			}
		} else {
			content = "Unable to add reviewer".to_string();
			invoke_ephermal(&content, &ctx, &command).await;
		}
	}
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
	let content: String;

	let actor_user_id = command.user.id.get();
	if actor_user_id != 164072941645070336 {
		content = "Forbidden".to_string();
		invoke_ephermal(&content, &ctx, &command).await;
	} else {
		let reviewer = &command.data.options();
		if let Some(ResolvedOption {
			value: ResolvedValue::User(user, _),
			..
		}) = reviewer.get(0)
		{
			let reviewer_service = ReviewerService::new();

			match reviewer_service.remove_reviewer(&ctx, &user).await {
				Ok(()) => {
					content = "User has been demoted from reviewer".to_string();
					invoke_ephermal(&content, &ctx, &command).await;

					{
						let mut log_message = MessageBuilder::new();
						log_message.push_bold(format!("{} ", &user.name));
						log_message.push_line(format!("({}) has been demoted from Reviewer", &user.id));
						log_to_discord(
							log_message.build(),
							ctx.clone(),
						).await
					}
				}
				Err(error) => {
					content = "Unable to remove reviewer".to_string();
					invoke_ephermal(&content, &ctx, &command).await;

					{
						{
							let mut log_message = MessageBuilder::new();
							log_message.push("Error demoting ".to_string());
							log_message.push_bold(format!("{} ", &user.name));
							log_message.push_line(format!("({}) from reviewer", &user.id));
							log_message.push_codeblock(format!("{:?}", error), Some("rust"));
							log_to_discord(
								log_message.build(),
								ctx.clone(),
							).await
						}
					}
				}
			}
		} else {
			content = "Unable to remove reviewer".to_string();
			invoke_ephermal(&content, &ctx, &command).await;
		}
	}
}
