use crate::request_manager::service::request_manager_service::RequestConfigService;
use crate::serenity::discord::{get_request_level_config_embed, get_request_level_embed};
use crate::{
	config::client_config::CLIENT_CONFIG,
	serenity::discord::{get_init_gd_account_link_embed},
};
use log::error;
use serenity::all::{CurrentUser, GenericChannelId};
use serenity::{
	all::{ButtonStyle, Context, CreateMessage, GetMessages, Message, User},
	builder::CreateButton,
};
use std::process;
use std::sync::OnceLock;

pub static REQUESTX_BOT_USER: OnceLock<CurrentUser> = OnceLock::new();

pub async fn init_request_message(ctx: &Context, bot_user: &User) {
	let request_channel = GenericChannelId::new(CLIENT_CONFIG.discord_public_channel_id);

	let messages = request_channel
		.messages(&ctx.http, GetMessages::new())
		.await
		.map_err(|read_messages_error| {
			error!(
				"Failed to retrieve messages from request channel: {}",
				read_messages_error
			);
			process::exit(1)
		})
		.ok()
		.unwrap_or_default();

	if !messages.is_empty() {
		let request_message_exists = messages.iter().fold(false, |request_exists, message| {
			request_exists || check_message(&message, "request".to_string())
		});

		if !request_message_exists {
			send_request_message(&ctx, &bot_user, &request_channel).await
		}
	} else {
		send_request_message(&ctx, &bot_user, &request_channel).await;
	}
}

pub async fn init_request_config_message(ctx: &Context, bot_user: &User) {
	let request_config_channel =
		GenericChannelId::new(CLIENT_CONFIG.discord_request_config_channel_id);

	let messages = request_config_channel
		.messages(&ctx.http, GetMessages::new())
		.await
		.map_err(|read_messages_error| {
			error!(
				"Failed to retrieve messages from request channel: {}",
				read_messages_error
			);
			process::exit(1)
		})
		.ok()
		.unwrap_or_default();

	if !messages.is_empty() {
		let request_config_message_exists =
			messages
				.iter()
				.fold(false, |request_config_exists, message| {
					request_config_exists || check_message(&message, "request-config".to_string())
				});

		if !request_config_message_exists {
			send_request_config_message(&ctx, &bot_user, &request_config_channel).await
		}
	} else {
		send_request_config_message(&ctx, &bot_user, &request_config_channel).await;
	}
}

pub async fn init_verify_message(ctx: &Context, bot_user: &User) {
	let verify_channel = GenericChannelId::new(CLIENT_CONFIG.discord_verify_channel_id);

	let messages = verify_channel
		.messages(&ctx.http, GetMessages::new())
		.await
		.map_err(|read_messages_error| {
			error!(
				"Failed to retrieve messages from verification channel: {}",
				read_messages_error
			);
			process::exit(1)
		})
		.ok()
		.unwrap_or_default();

	if !messages.is_empty() {
		let init_message_exists =
			messages
				.iter()
				.fold(false, | init_exists, message| {
					init_exists || check_message(&message, "init".to_string())
				});

		if !init_message_exists {
			send_init_message(&ctx, &bot_user, &verify_channel).await
		}
	} else {
		send_init_message(&ctx, &bot_user, &verify_channel).await;
	}
}

fn check_message(message: &Message, condition_string: String) -> bool {
	if let Some(embed) = message.embeds.get(0) {
		if let Some(footer) = &embed.footer {
			return footer.text == condition_string;
		}
	}

	false
}

async fn send_request_message(ctx: &Context, bot_user: &User, request_channel: &GenericChannelId) {
	let init_message = CreateMessage::new()
		.button(
			CreateButton::new("request-level-button")
				.label("Request a Level")
				.style(ButtonStyle::Success),
		)
		.button(
			CreateButton::new("edit-level-button")
				.label("Edit Existing Level Request")
				.style(ButtonStyle::Primary)
		)
		.embed(get_request_level_embed(&bot_user));

	request_channel
		.send_message(&ctx.http, init_message)
		.await
		.map_err(|send_init_message_error| {
			error!(
				"Failed to send request message to request channel: {}",
				send_init_message_error
			);
		})
		.ok();
}

async fn send_request_config_message(
	ctx: &Context,
	bot_user: &User,
	request_channel: &GenericChannelId,
) {
	let request_config_service = RequestConfigService::new();
	let request_config = request_config_service
		.get_request_config()
		.await
		.unwrap_or_else(|get_request_config_error| {
			error!(
				"Unable to retrieve request config: {}",
				get_request_config_error
			);
			panic!(
				"Unable to retrieve request config: {}",
				get_request_config_error
			);
		});
	let init_message =
		CreateMessage::new().embed(get_request_level_config_embed(&bot_user, &request_config));

	request_channel
		.send_message(&ctx.http, init_message)
		.await
		.map_err(|send_init_message_error| {
			error!(
				"Failed to send request config to request channel: {}",
				send_init_message_error
			);
		})
		.ok();
}

async fn send_init_message(ctx: &Context, bot_user: &User, verify_channel: &GenericChannelId) {
	let init_message = CreateMessage::new()
		.button(
			CreateButton::new("init-gd-account-link-button")
				.label("Link GD Account")
				.style(ButtonStyle::Success),
		)
		.button(
			CreateButton::new("verify-gd-account-link-button")
				.label("Verify GD Account Link")
				.style(ButtonStyle::Primary),
		)
		.embed(get_init_gd_account_link_embed(&bot_user));

	verify_channel
		.send_message(&ctx.http, init_message)
		.await
		.map_err(|send_init_message_error| {
			error!(
				"Failed to send init message to verification channel: {}",
				send_init_message_error
			);
		})
		.ok();
}