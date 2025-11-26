use serenity::{
	all::{CommandInteraction, CommandOptionType, Context},
	builder::{CreateCommand, CreateCommandOption}
};

use crate::{
	request_manager::{
		model::request_manager::UpdateRequestManagerRequest,
		service::request_manager_service::RequestManagerService
	},
	serenity::discord::invoke_command_ephemeral
};
use crate::serenity::discord::extract_command_options;

pub fn register_request_manager<'a>() -> CreateCommand<'a> {
	CreateCommand::new("request-manager")
		.description("Configure options for leve requests.")
		.add_option(CreateCommandOption::new(
			CommandOptionType::Integer,
			"request-cooldown",
			"Set the request cooldown in minutes"
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"enable-requests",
			"Enable/disable all level requests"
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"enable-gd-requests",
			"Enable/disable all GD http requests"
		))
		.add_option(CreateCommandOption::new(
			CommandOptionType::Boolean,
			"allow-non-user-created-levels",
			"Enable/disable ability to request levels created by the requestor"
		))
}

pub async fn run_request_manager(ctx: &Context, command: &CommandInteraction) {
	let command_map = extract_command_options(&command);
	let duration_in_minutes = if let Some(duration_in_minutes_input) = command_map
		.get("request-cooldown") {
		Some(duration_in_minutes_input.as_i64().unwrap().unsigned_abs())
	} else {None};
	let enable_requests = if let Some(enable_requests_input) = command_map
		.get("enable-requests") {
		Some(enable_requests_input.as_bool().unwrap())
	} else {None};
	let enable_gd_requests = if let Some(enable_gd_requests_input) = command_map
		.get("enable-gd-requests") {
		Some(enable_gd_requests_input.as_bool().unwrap())
	} else {None};
	let allow_non_user_created_levels = if let Some(allow_non_user_created_levels_input) = command_map
		.get("allow-non-user-created-levels") {
		Some(allow_non_user_created_levels_input.as_bool().unwrap())
	} else {None};

	let update_request_manager_request = UpdateRequestManagerRequest::new(
		duration_in_minutes,
		enable_requests,
		enable_gd_requests,
		allow_non_user_created_levels,
	);

	let service = RequestManagerService::new();

	match service
		.update_request_manager(&update_request_manager_request)
		.await {
		Ok(()) => {
			invoke_command_ephemeral(
				&format!(
					"{}.",
					build_request_manager_update_string(&update_request_manager_request)
				),
				&ctx,
				&command
			).await;
		}
		Err(error) => {
			invoke_command_ephemeral(&error.to_string(), &ctx, &command).await;
		}
	}
}

fn build_request_manager_update_string(update_request_manager_request: &UpdateRequestManagerRequest) -> String {
	let mut command_ephemeral_content: Vec<String> = Vec::new();

	if let Some(duration_in_minutes) = update_request_manager_request.duration_in_minutes {
		command_ephemeral_content.push(format_cooldown_duration_string(duration_in_minutes));
	}
	if let Some(enable_requests) = update_request_manager_request.enable_requests {
		command_ephemeral_content.push(format!(
			"Level requests have been **{}**",
			if enable_requests { "enabled" } else { "disabled" }
		))
	}
	if let Some(enable_gd_requests) = update_request_manager_request.enable_gd_requests {
		command_ephemeral_content.push(format!(
			"GD HTTPS requests have been **{}**",
			if enable_gd_requests { "enabled" } else { "disabled" }
		))
	}
	if let Some(allow_non_user_created_levels) = update_request_manager_request.allow_non_user_created_levels {
		command_ephemeral_content.push(format!(
			"Allow non user created level requests have been **{}**",
			if allow_non_user_created_levels { "enabled" } else { "disabled" }
		))
	}

	command_ephemeral_content.join(".\n")
}

fn format_cooldown_duration_string(cooldown_duration_in_minutes: u64) -> String {
	let mut content = String::new();
	if cooldown_duration_in_minutes == 0 {
		content.push_str("Request cooldown has been **disabled**");
	} else {
		content.push_str("Request cooldown has been set to ");
		let days = cooldown_duration_in_minutes / 1440;
		let hours = (cooldown_duration_in_minutes % 1440) / 60;
		let minutes = cooldown_duration_in_minutes % 60;
		let mut time_parts = Vec::new();

		push_time_unit(&mut time_parts, days, "day", "days");
		push_time_unit(&mut time_parts, hours, "hour", "hours");
		push_time_unit(&mut time_parts, minutes, "minute", "minutes");

		let duration_str = if time_parts.len() > 1 {
			let last_unit = time_parts.pop().unwrap();
			let remaining_time_units = time_parts.join(", ");
			format!("{} and {}", remaining_time_units, last_unit)
		} else {
			time_parts.join(", ")
		};
		content.push_str(&format!("{}", duration_str));
	}

	content
}

fn push_time_unit(time_parts: &mut Vec<String>, value: u64, singular: &str, plural: &str) {
	if value == 1 {
		time_parts.push(format!("**1 {}**", singular));
	} else if value > 1 {
		time_parts.push(format!("**{} {}**", value, plural));
	}
}