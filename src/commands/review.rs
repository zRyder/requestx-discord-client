use serenity::{
	all::{
		ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand,
		CreateCommandOption
	},
	builder::CreateThread,
	Error
};

use crate::{
	model::{level_request::GetLevelRequest, requestx_api::level_request_data::LevelRequestData},
	service::level_review_service::LevelReviewService,
	util::discord::invoke_ephermal
};

pub fn register() -> CreateCommand {
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

pub async fn run(ctx: &Context, command: &CommandInteraction) {
	let mut content: String;
	if !command
		.user
		.has_role(&ctx.http, 1192954008013385839, 1192955803418775612)
		.await
		.unwrap()
	{
		content = "Unauthorized".to_string();
		invoke_ephermal(&content, &ctx, &command).await;
	} else {
		let mut update_flag = false;
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
