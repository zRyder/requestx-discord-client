use serenity::{
	all::{CommandInteraction, CommandOptionType, Context},
	builder::{CreateCommand, CreateCommandOption}
};

use crate::{
	request_manager::{
		model::request_manager::UpdateRequestManager,
		service::request_manager_service::RequestManagerService
	},
	serenity::discord::invoke_ephemeral
};

pub fn register_request_manager() -> CreateCommand {
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
}

pub async fn run_request_manager(ctx: &Context, command: &CommandInteraction) {
	let update_request_manager_request = UpdateRequestManager {
		duration_in_minutes: if let Some(request_cooldown_command_option) = command
			.data
			.options
			.iter()
			.find(|command_option| command_option.name.eq("request-cooldown"))
		{
			Some(
				request_cooldown_command_option
					.value
					.as_i64()
					.unwrap()
					.unsigned_abs()
			)
		} else {
			None
		},
		enable_requests: if let Some(enable_requests_command_option) = command
			.data
			.options
			.iter()
			.find(|command_option| command_option.name.eq("enable-requests"))
		{
			Some(enable_requests_command_option.value.as_bool().unwrap())
		} else {
			None
		},
		enable_gd_requests: if let Some(enable_gd_requests_command_option) = command
			.data
			.options
			.iter()
			.find(|command_option| command_option.name.eq("enable-gd-requests"))
		{
			Some(enable_gd_requests_command_option.value.as_bool().unwrap())
		} else {
			None
		}
	};

	let service = RequestManagerService::new();

	match service
		.update_request_manager(&update_request_manager_request)
		.await
	{
		Ok(()) => {
			let mut string_content: Vec<String> = Vec::new();
			if update_request_manager_request.duration_in_minutes.is_some() {
				let mut content = String::new();
				let minutes = update_request_manager_request.duration_in_minutes.unwrap();
				if minutes == 0 {
					content.push_str("Request cooldown has been **disabled**");
				} else {
					content.push_str("Request cooldown has been set to ");
					let hours = minutes / 60;
					let remaining_minutes = minutes % 60;
					let mut parts = Vec::new();

					if hours > 0 {
						if hours == 1 {
							parts.push("**1 hour**".to_string());
						} else {
							parts.push(format!("**{} hours**", hours));
						}
					}

					if remaining_minutes > 0 {
						if remaining_minutes == 1 {
							parts.push("**1 minute**".to_string());
						} else {
							parts.push(format!("**{} minutes**", remaining_minutes));
						}
					}

					content.push_str(&format!("{}", &parts.join(" and ")));
				}
				string_content.push(content)
			}
			if update_request_manager_request.enable_requests.is_some() {
				string_content.push(format!(
					"Level requests have been **{}**",
					if update_request_manager_request.enable_requests.unwrap() {
						"enabled"
					} else {
						"disabled"
					}
				))
			}
			if update_request_manager_request.enable_gd_requests.is_some() {
				string_content.push(format!(
					"GD HTTPS requests have been **{}**",
					if update_request_manager_request.enable_gd_requests.unwrap() {
						"enabled"
					} else {
						"disabled"
					}
				))
			}
			invoke_ephemeral(&format!("{}.", string_content.join(".\n")), &ctx, &command).await;
		}
		Err(error) => {
			invoke_ephemeral(&error.to_string(), &ctx, &command).await;
		}
	}
}
