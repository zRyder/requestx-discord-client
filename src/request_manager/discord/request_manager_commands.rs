use crate::config::client_config::CLIENT_CONFIG;
use crate::config::discord_config::REQUESTX_BOT_USER;
use crate::serenity::discord::{extract_command_options, get_request_level_config_embed};
use crate::{
	request_manager,
	request_manager::{
		model::request_manager::RequestConfig,
		service::request_manager_service::RequestConfigService,
	},
	serenity::discord::invoke_command_ephemeral,
};
use log::error;
use serenity::all::{EditMessage, GenericChannelId, GetMessages, Message};
use serenity::builder::CreateMessage;
use serenity::{
	all::{CommandInteraction, CommandOptionType, Context},
	builder::{CreateCommand, CreateCommandOption},
};

pub fn register_request_manager<'a>() -> CreateCommand<'a> {
	CreateCommand::new("request-manager")
		.description("Configure options for leve requests.")
		.add_option(CreateCommandOption::new(
			CommandOptionType::Integer,
			"request-cooldown",
			"Set the request cooldown in minutes",
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"enable-requests",
			"Enable/disable all level requests",
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"enable-gd-requests",
			"Enable/disable all GD http requests",
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"allow-non-user-created-levels",
			"Enable/disable ability to request levels created by the requestor",
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"allow-platformer-levels",
			"Enable/disable ability to request platformer levels",
		))
}

pub async fn run_request_manager(ctx: &Context, command: &CommandInteraction) {
	let command_map = extract_command_options(&command);
	let duration_in_minutes =
		if let Some(duration_in_minutes_input) = command_map.get("request-cooldown") {
			Some(duration_in_minutes_input.as_i64().unwrap().unsigned_abs())
		} else {
			None
		};
	let enable_requests = if let Some(enable_requests_input) = command_map.get("enable-requests") {
		Some(enable_requests_input.as_bool().unwrap())
	} else {
		None
	};
	let enable_gd_requests =
		if let Some(enable_gd_requests_input) = command_map.get("enable-gd-requests") {
			Some(enable_gd_requests_input.as_bool().unwrap())
		} else {
			None
		};
	let allow_non_user_created_levels = if let Some(allow_non_user_created_levels_input) =
		command_map.get("allow-non-user-created-levels") {
		Some(allow_non_user_created_levels_input.as_bool().unwrap())
	} else {
		None
	};
	let allow_platformer_levels = if let Some(allow_platformer_levels_input) =
		command_map.get("allow-platformer-levels") {
		Some(allow_platformer_levels_input.as_bool().unwrap())
	} else {
		None
	};

	let update_request_manager_request = RequestConfig::new(
		duration_in_minutes,
		enable_requests,
		enable_gd_requests,
		allow_non_user_created_levels,
		allow_platformer_levels
	);

	let service = RequestConfigService::new();

	match service
		.update_request_config(&update_request_manager_request)
		.await {
		Ok(()) => {
			send_or_edit_request_config_message(&ctx).await;
			invoke_command_ephemeral(
				&format!(
					"{}.",
					build_request_manager_update_string(&update_request_manager_request)
				),
				&ctx,
				&command,
			)
			.await;
		}
		Err(error) => {
			invoke_command_ephemeral(&error.to_string(), &ctx, &command).await;
		}
	}
}

fn build_request_manager_update_string(update_request_manager_request: &RequestConfig) -> String {
	let mut command_ephemeral_content: Vec<String> = Vec::new();

	if let Some(duration_in_minutes) = update_request_manager_request.duration_in_minutes {
		if let Some(cooldown_string) =
			request_manager::format_cooldown_duration_string(duration_in_minutes) {
			command_ephemeral_content.push(format!(
				"Request cooldown has been set to **{}**",
				cooldown_string
			));
		} else {
			command_ephemeral_content.push("Request cooldown has been **disabled**".to_string());
		}
	}
	if let Some(enable_requests) = update_request_manager_request.enable_requests {
		command_ephemeral_content.push(format!(
			"Level requests have been **{}**",
			if enable_requests {
				"enabled"
			} else {
				"disabled"
			}
		))
	}
	if let Some(enable_gd_requests) = update_request_manager_request.enable_gd_requests {
		command_ephemeral_content.push(format!(
			"GD HTTPS requests have been **{}**",
			if enable_gd_requests {
				"enabled"
			} else {
				"disabled"
			}
		))
	}
	if let Some(allow_non_user_created_levels) =
		update_request_manager_request.allow_non_user_created_levels {
		command_ephemeral_content.push(format!(
			"Allow non user created level requests have been **{}**",
			if allow_non_user_created_levels {
				"enabled"
			} else {
				"disabled"
			}
		))
	}
	if let Some(allow_platformer_levels) =
		update_request_manager_request.allow_platformer_levels {
		command_ephemeral_content.push(format!(
			"Platformer level requests have been **{}**",
			if allow_platformer_levels {
				"enabled"
			} else {
				"disabled"
			}
		))
	}

	command_ephemeral_content.join(".\n")
}

async fn send_or_edit_request_config_message(ctx: &Context) {
	let request_config_channel =
		GenericChannelId::new(CLIENT_CONFIG.discord_request_config_channel_id);
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

	if let Some(mut existing_request_config_message) =
		get_request_config_message(&ctx, &request_config_channel).await {
		let request_config_message = EditMessage::new().embed(get_request_level_config_embed(
			REQUESTX_BOT_USER.get().unwrap(),
			&request_config,
		));

		if let Err(edit_message_error) = existing_request_config_message
			.edit(&ctx.http, request_config_message)
			.await {
			error!(
				"Unable to edit request config message: {}",
				edit_message_error
			);
		}
	} else {
		let request_config_message = CreateMessage::new().embed(get_request_level_config_embed(
			REQUESTX_BOT_USER.get().unwrap(),
			&request_config,
		));

		if let Err(edit_message_error) = request_config_channel
			.send_message(&ctx.http, request_config_message)
			.await {
			error!(
				"Unable to send request config message: {}",
				edit_message_error
			);
		}
	};
}

async fn get_request_config_message(
	ctx: &Context,
	request_config_channel: &GenericChannelId,
) -> Option<Message> {
	request_config_channel
		.messages(&ctx.http, GetMessages::new())
		.await
		.ok()?
		.into_iter()
		.next()
}
