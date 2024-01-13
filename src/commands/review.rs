use serenity::all::{
	CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption
};

use crate::{
	config::client_config::CLIENT_CONFIG, service::level_review_service::LevelReviewService,
	util::discord::invoke_ephermal
};

pub fn register_review() -> CreateCommand {
	CreateCommand::new("review")
		.description("Submit a review for the given level")
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"level-id",
				"The level ID of the request to review."
			)
			.required(true)
		)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::String,
				"review-contents",
				"The review to be shared with the Discord user who requested the level."
			)
			.required(true)
		)
}

pub async fn post_level_review(ctx: &Context, command: &CommandInteraction) {
	let content: String;
	if !command
		.user
		.has_role(
			&ctx.http,
			CLIENT_CONFIG.discord_guild_id,
			CLIENT_CONFIG.discord_reviewer_role_id
		)
		.await
		.unwrap()
	{
		content = "Forbidden".to_string();
		invoke_ephermal(&content, &ctx, &command).await;
	} else {
		let reviewer_discord_user_id = command.user.id.get();
		let level_id = command
			.data
			.options
			.get(0)
			.unwrap()
			.value
			.as_i64()
			.unwrap()
			.unsigned_abs();
		let review_contents = command
			.data
			.options
			.get(1)
			.unwrap()
			.value
			.as_str()
			.unwrap()
			.to_string();
		let level_review_service = LevelReviewService::new();

		match level_review_service
			.review_level(
				&ctx,
				&command,
				level_id,
				reviewer_discord_user_id,
				review_contents
			)
			.await
		{
			Ok(message_string) => invoke_ephermal(&message_string, &ctx, &command).await,
			Err(level_review_error) => {
				invoke_ephermal(&level_review_error.to_string(), &ctx, &command).await
			}
		}
	}
}
