use crate::config::client_config::CLIENT_CONFIG;
use crate::config::constants::REVIEW_LOG_MESSAGE_LIMIT;
use crate::level_request::model::level_request::LevelRequest;
use crate::level_review::model::level_review::{LevelReview, UpdateLevelReviewMessageId};
use crate::level_review::service::level_review_service::LevelReviewService;
use crate::serenity::discord::{invoke_message_response, log_to_discord};
use log::error;
use serenity::all::{
	Color, Context, CreateEmbed, CreateMessage, EditMessage, GenericChannelId, GuildId,
	Mentionable, Message, MessageBuilder, MessageId, RoleId, User, UserId,
};

pub async fn post_level_review(ctx: &Context, review_message: &Message, thread_name: &str) {
	let message_response: &str;
	let reviewer_user = &review_message.author;
	let reviewer_discord_user_id = review_message.author.id.get();
	if !reviewer_user
		.has_role(
			&ctx.http,
			GuildId::new(CLIENT_CONFIG.discord_guild_id),
			RoleId::new(CLIENT_CONFIG.discord_reviewer_role_id),
		)
		.await
		.unwrap()
	{
		message_response = "Forbidden to use this service.";
		return invoke_message_response(message_response, &ctx, &reviewer_user).await;
	}

	let Some(level_id) = extract_level_id(thread_name) else {
		message_response = "Unable to find level request to review.";
		error!("Unable to parse level ID from thread name: {}", thread_name);
		return invoke_message_response(message_response, &ctx, &reviewer_user).await;
	};
	let review_contents = review_message.content.to_string();
	let level_review_service = LevelReviewService::new();

	let level_review = match level_review_service
		.get_level_review(reviewer_discord_user_id, level_id)
		.await
	{
		Ok(Some(mut existing_level_review)) => {
			existing_level_review.review_contents = review_contents;
			existing_level_review
		}
		Ok(None) => LevelReview::new(reviewer_discord_user_id, None, level_id, review_contents),
		Err(get_level_review_error) => {
			error!(
				"Error checking for existing level review: {:?}",
				get_level_review_error
			);
			message_response = "An unknown error occurred.";
			return invoke_message_response(message_response, &ctx, &reviewer_user).await;
		}
	};

	let reviewed_level_request = match level_review_service.review_level(&level_review).await {
		Ok(level_request) => level_request,
		Err(review_level_error) => {
			error!("Error reviewing level: {:?}", review_level_error);
			message_response = "The server failed to write the level review.";
			return invoke_message_response(message_response, &ctx, &reviewer_user).await;
		}
	};

	send_level_review_message(
		&reviewer_user,
		&ctx,
		&reviewed_level_request,
		&level_review_service,
		&level_review,
	)
	.await;

	message_response = "Review submitted!";
	log_to_discord(
		ctx.clone(),
		build_log_message(&reviewer_user, level_id, &level_review),
	)
	.await;
	invoke_message_response(message_response, &ctx, &reviewer_user).await;
}

fn extract_level_id(thread_name: &str) -> Option<u64> {
	let trimmed = thread_name.trim();

	// Level request was made with GD requests off
	if trimmed.chars().all(|c| c.is_ascii_digit()) {
		return trimmed.parse::<u64>().ok();
	}

	// Level request was made with GD requests off
	let start = trimmed.find('(')? + 1;
	let end = trimmed.find(')')?;
	trimmed[start..end].trim().parse::<u64>().ok()
}

fn build_log_message(reviewer: &User, level_id: u64, level_review: &LevelReview) -> String {
	let mut log_message = MessageBuilder::new();
	log_message = log_message.push_bold(format!("{} ", reviewer.name).as_str());
	log_message = log_message.push_line(
		format!(
			"({}) left a review on level request ID: {}",
			reviewer.id, level_id
		)
		.as_str(),
	);
	let mut truncated_level_review = level_review.review_contents.clone();
	if level_review.review_contents.len() > REVIEW_LOG_MESSAGE_LIMIT {
		truncated_level_review.truncate(REVIEW_LOG_MESSAGE_LIMIT);
		truncated_level_review.push_str("... [truncated due to word count]")
	}
	log_message = log_message.push_codeblock(
		format!("{}: {}", level_id, truncated_level_review).as_str(),
		Some("rust"),
	);
	log_message.build()
}

async fn send_level_review_message<'a>(
	reviewer: &User,
	ctx: &Context,
	reviewed_level_request: &LevelRequest,
	level_review_service: &LevelReviewService<'a>,
	level_review: &LevelReview,
) {
	let discord_ephemeral_message;
	let (review_message, level_review_embed) =
		build_level_review_message(reviewer, &level_review, &reviewed_level_request);

	let reviewed_level_request_thread =
		GenericChannelId::new(reviewed_level_request.discord_message_id.unwrap());
	if level_review.discord_message_id.is_some() {
		if let Err(edit_message_error) = reviewed_level_request_thread
			.edit_message(
				&ctx.http,
				MessageId::new(level_review.discord_message_id.unwrap()),
				EditMessage::new()
					.content(&review_message.build())
					.embed(level_review_embed),
			)
			.await
		{
			error!("Unable to edit review message: {}", edit_message_error);
			let discord_ephemeral_message = "An unknown error occurred.";
			invoke_message_response(discord_ephemeral_message, &ctx, &reviewer).await;
		}
	} else {
		match reviewed_level_request_thread
			.send_message(
				&ctx.http,
				CreateMessage::new()
					.content(&review_message.build())
					.embed(level_review_embed),
			)
			.await
		{
			Ok(message) => {
				update_level_review_message_id(level_review_service, level_review, message).await;
			}
			Err(send_message_error) => {
				error!("Unable to edit review message: {}", send_message_error);
				discord_ephemeral_message = "An unknown error occurred.";
				invoke_message_response(discord_ephemeral_message, &ctx, &reviewer).await;
			}
		};
	}
}

async fn update_level_review_message_id<'a>(
	level_review_service: &LevelReviewService<'a>,
	level_review: &LevelReview,
	message: Message,
) {
	let update_level_review_message_id_request = UpdateLevelReviewMessageId::new(
		level_review.discord_user_id,
		level_review.level_id,
		message.id.get(),
	);
	if let Err(update_level_review_message_id) = level_review_service
		.update_level_review_message_id(update_level_review_message_id_request)
		.await
	{
		error!(
			"Unable to update level review message id: {:?}",
			update_level_review_message_id
		);
	}
}

fn build_level_review_message<'a>(
	reviewer: &User,
	level_review: &'a LevelReview,
	reviewed_level_request: &LevelRequest,
) -> (MessageBuilder, CreateEmbed<'a>) {
	let mut review_message = MessageBuilder::new();

	if reviewed_level_request.notify {
		review_message = review_message.push_line(
			format!(
				"{}",
				UserId::new(reviewed_level_request.discord_user_id).mention()
			)
			.as_str(),
		);
		review_message = review_message.push_line("");
	}

	review_message = review_message.push_bold_line(
		format!("Your level has been reviewed by {}", reviewer.id.mention()).as_str(),
	);

	let mut level_review_embed = CreateEmbed::new().color(Color::BLUE);
	for paragraph in level_review
		.review_contents
		.lines()
		.filter(|line| !line.trim().is_empty())
	{
		level_review_embed = level_review_embed.field("", paragraph, false);
	}
	(review_message, level_review_embed)
}
