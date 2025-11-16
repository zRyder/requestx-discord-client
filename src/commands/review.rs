use log::error;
use serenity::all::{ChannelId, Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateMessage, EditMessage, Mentionable, Message, MessageBuilder, UserId};

use crate::{
	config::client_config::CLIENT_CONFIG,
	model::level_review::{LevelReview, UpdateLevelReviewMessageId},
	serenity::discord::{invoke_ephemeral, log_to_discord},
	service::level_review_service::LevelReviewService
};
use crate::model::level_request::LevelRequest;

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
	let discord_ephemeral_message: &str;
	let command_user = &command.user;
	if !command_user
		.has_role(
			&ctx.http,
			CLIENT_CONFIG.discord_guild_id,
			CLIENT_CONFIG.discord_reviewer_role_id
		)
		.await
		.unwrap()
	{
		discord_ephemeral_message = "Forbidden";
		return invoke_ephemeral(discord_ephemeral_message, &ctx, &command).await;
	}

	let reviewer_discord_user_id = command_user.id.get();
	let command_options = &command.data.options;
	let level_id = command_options
		.get(0)
		.unwrap()
		.value
		.as_i64()
		.unwrap()
		.unsigned_abs();
	let review_contents = command_options
		.get(1)
		.unwrap()
		.value
		.as_str()
		.unwrap()
		.to_string();
	let level_review_service = LevelReviewService::new();

	let level_review = match level_review_service
		.get_level_review(reviewer_discord_user_id, level_id)
		.await {
		Ok(Some(mut existing_level_review)) => {
			existing_level_review.review_contents = review_contents;
			existing_level_review
		},
		Ok(None) => LevelReview::new(reviewer_discord_user_id, None, level_id, review_contents),
		Err(get_level_review_error) => {
			error!(
				"Error checking for existing level review: {:?}",
				get_level_review_error
			);
			discord_ephemeral_message = "An unknown error occurred.";
			return invoke_ephemeral(discord_ephemeral_message, &ctx, &command).await;
		}
	};

	let reviewed_level_request = match level_review_service.review_level(&level_review).await {
		Ok(level_request) => level_request,
		Err(review_level_error) => {
			error!("Error reviewing level: {:?}", review_level_error);
			discord_ephemeral_message = "The server failed to write the level review.";
			return invoke_ephemeral(discord_ephemeral_message, &ctx, &command).await;
		}
	};

	send_level_review_message(
		&command,
		&ctx,
		&reviewed_level_request,
		&level_review_service,
		&level_review
	).await;

	discord_ephemeral_message = "Review submitted!";
	log_to_discord(
		ctx.clone(),
		build_log_message(
			&command,
			level_id,
			&level_review
		)
	).await;
	invoke_ephemeral(discord_ephemeral_message, &ctx, &command).await;
}

fn build_log_message(command: &CommandInteraction, level_id: u64, level_review: &LevelReview) -> String {
	let mut log_message = MessageBuilder::new();
	log_message.push_bold(format!("{} ", command.user.name));
	log_message.push_line(format!(
		"({}) left a review on level request ID: {}",
		command.user.id, level_id
	));
	log_message.push_codeblock(
		format!("{}: {}", level_id, &level_review.review_contents),
		Some("rust")
	);
	log_message.build()
}

async fn send_level_review_message<'a>(
	command: &CommandInteraction,
	ctx: &Context,
	reviewed_level_request: &LevelRequest,
	level_review_service: &LevelReviewService<'a>,
	level_review: &LevelReview
) {
	let discord_ephemeral_message;
	let (mut review_message, level_review_embed) =
		build_level_review_message(command, &level_review, &reviewed_level_request);

	let reviewed_level_request_thread =
		ChannelId::new(reviewed_level_request.discord_message_id.unwrap());
	if level_review.discord_message_id.is_some() {
		if let Err(edit_message_error) = reviewed_level_request_thread
			.edit_message(
				&ctx.http,
				level_review.discord_message_id.unwrap(),
				EditMessage::new()
					.content(&review_message.build())
					.embed(level_review_embed)
			)
			.await {
			error!("Unable to edit review message: {}", edit_message_error);
			let discord_ephemeral_message = "An unknown error occurred.";
			invoke_ephemeral(discord_ephemeral_message, &ctx, &command).await;
		}
	} else {
		match reviewed_level_request_thread
			.send_message(
				&ctx.http,
				CreateMessage::new()
					.content(&review_message.build())
					.embed(level_review_embed)
			)
			.await {
			Ok(message) => {
				update_level_review_message_id(level_review_service, level_review, message).await;
			}
			Err(send_message_error) => {
				error!("Unable to edit review message: {}", send_message_error);
				discord_ephemeral_message = "An unknown error occurred.";
				invoke_ephemeral(discord_ephemeral_message, &ctx, &command).await;
			}
		};
	}
}

async fn update_level_review_message_id<'a>(
	level_review_service: &LevelReviewService<'a>,
	level_review: &LevelReview,
	message: Message
) {
	let update_level_review_message_id_request = UpdateLevelReviewMessageId::new(
		level_review.discord_user_id,
		level_review.level_id,
		message.id.get()
	);
	if let Err(update_level_review_message_id) = level_review_service
		.update_level_review_message_id(update_level_review_message_id_request)
		.await {
		error!(
			"Unable to update level review message id: {:?}",
			update_level_review_message_id
		);
	}
}

fn build_level_review_message(command: &CommandInteraction, level_review: &LevelReview, reviewed_level_request: &LevelRequest) -> (MessageBuilder, CreateEmbed) {
	let mut review_message = MessageBuilder::new();

	if reviewed_level_request.notify {
		review_message.push_line(format!(
			"{}",
			UserId::new(reviewed_level_request.discord_user_id).mention()
		));
		review_message.push_line("");
	}

	review_message.push_bold_line(format!(
		"Your level has been reviewed by {}",
		command.user.id.mention()
	));

	let mut level_review_embed = CreateEmbed::new().color(Color::BLUE);
	for paragraph in level_review
		.review_contents
		.lines()
		.filter(|line| !line.trim().is_empty()) {
		level_review_embed = level_review_embed.field("", paragraph, false);
	}
	(review_message, level_review_embed)
}
