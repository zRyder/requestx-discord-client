use async_trait::async_trait;
use log::{debug, error, info};
use serenity::{
	all::{
		ComponentInteraction, CreateInteractionResponse, GuildId, Interaction, Message,
		MessageType
	},
	prelude::{Context, EventHandler}
};
use serenity::all::{FullEvent, ModalInteraction, RoleId};
use crate::{
	config::{client_config::CLIENT_CONFIG, discord_config::init_verify_message},
	level_request::discord::request_level_command,
	level_review::discord::review,
	request_manager::discord::request_manager_commands,
	reviewer::discord::reviewer,
	send_level::discord::send_level,
	serenity::modals::get_init_gd_account_link_modal,
	user::discord::user_commands
};
use crate::config::discord_config::init_request_message;
use crate::level_request::discord::request_level_modal;
use crate::serenity::modals::get_request_level_modal;
use crate::user::discord::{user_buttons, user_modals};

pub struct Handler;


#[async_trait]
impl EventHandler for Handler {
	async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
		match event {
			FullEvent::Message { new_message, .. } => {
				if new_message.kind.eq(&MessageType::ThreadCreated)
					|| new_message
					.channel_id
					.eq(&CLIENT_CONFIG.discord_public_channel_id) {
					if !is_immune_from_message_deletion(&ctx, &new_message)
						.await {
						if let Err(message_delete_error) = new_message.delete(&ctx.http, None).await {
							error!("Unable to delete message: {}", message_delete_error);
						}
					}
				}
			}
			FullEvent::Ready { data_about_bot, ..} => {
				let user = &data_about_bot.user;
				info!("{} is connected!", user.name.as_str());
				let guild_id = GuildId::new(CLIENT_CONFIG.discord_guild_id);
				init_request_message(&ctx, &user).await;
				init_verify_message(&ctx, &user).await;

				guild_id
					.set_commands(
						&ctx.http,
						&[
							request_level_command::register_request_level(),
							request_level_command::register_edit_level_request(),
							request_level_command::register_delete_level_request(),
							review::register_review(),
							reviewer::register_add_reviewer(),
							reviewer::register_remove_reviewer(),
							send_level::register_send_level(),
							request_manager_commands::register_request_manager(),
							user_commands::register_view_cooldown(),
							user_commands::register_view_user_cooldown(),
						]
					)
					.await
					.expect("Unable to set commands");
			}
			FullEvent::InteractionCreate { interaction, .. } => {
				if let Interaction::Command(command) = interaction {
					debug!("Received command interaction: {command:#?}");

					match command.data.name.as_str() {
						"view-cooldown" => user_commands::run_view_cooldown(&ctx, &command).await,
						"view-user-cooldown" => user_commands::run_view_user_cooldown(&ctx, &command).await,
						"request-level" => request_level_command::run_request_level(&ctx, &command).await,
						"edit-level-request" => request_level_command::run_edit_level_request(&ctx, &command).await,
						"delete-level-request" => {
							request_level_command::run_delete_level_request(&ctx, &command).await
						}
						"review" => review::post_level_review(&ctx, &command).await,
						"add-reviewer" => reviewer::run_add_reviewer(&ctx, &command).await,
						"remove-reviewer" => reviewer::run_remove_reviewer(&ctx, &command).await,
						"send-level" => send_level::run_send_level(&ctx, &command).await,
						"request-manager" => request_manager_commands::run_request_manager(&ctx, &command).await,
						_ => println!("Unreachable")
					};
				} else if let Interaction::Component(component_interaction) = interaction {
					debug!("Received component interaction: {component_interaction:#?}");
					let component_interaction_id = &component_interaction.data.custom_id;
					let component_interaction_type = component_interaction_id
						.split("-")
						.collect::<Vec<&str>>()
						.last()
						.unwrap_or(&"")
						.to_owned();

					if component_interaction_type == "button" {
						handle_button_interactions(&ctx, &component_interaction, &component_interaction_id)
							.await
					}
				} else if let Interaction::Modal(modal_interaction) = interaction {
					handle_modal_interactions(&ctx, &modal_interaction).await
				} else {
					eprintln!("Unknown interaction type")
				}
			}
			_ => {}
		}
	}
}

async fn is_immune_from_message_deletion(
	ctx: &Context,
	message: &Message
) -> bool {
	let is_maintenance_user = message
		.author
		.has_role(
			&ctx.http,
			GuildId::new(CLIENT_CONFIG.discord_guild_id),
			RoleId::new(CLIENT_CONFIG.discord_maintenance_role_id)
		)
		.await
		.unwrap_or(false);
	let is_bot_user = message
		.author
		.id
		.get()
		.eq(&CLIENT_CONFIG.discord_app_id);

	is_maintenance_user || is_bot_user
}

async fn handle_button_interactions(
	ctx: &Context,
	button_interaction: &ComponentInteraction,
	button_interaction_id: &str
) {
	match button_interaction_id {
		"request-level-button" => {
			if let Err(create_modal_error) = button_interaction
				.create_response(
					&ctx.http,
					CreateInteractionResponse::Modal(get_request_level_modal())
				)
				.await {
				error!(
					"Unable to create request level modal: {}",
					create_modal_error
				);
			}
		}
		"init-gd-account-link-button" => {
			if let Err(create_modal_error) = button_interaction
				.create_response(
					&ctx.http,
					CreateInteractionResponse::Modal(get_init_gd_account_link_modal())
				)
				.await {
				error!(
					"Unable to create init gd account link modal: {}",
					create_modal_error
				);
			}
		}
		"verify-gd-account-link-button" => { user_buttons::run_init_gd_account_link_modal(&ctx, &button_interaction).await }
		_ => println!("Unreachable")
	};
}

async fn handle_modal_interactions(
	ctx: &Context,
	modal_interaction: &ModalInteraction
) {
	match modal_interaction.data.custom_id.as_str() {
		"init-gd-account-link-modal" => user_modals::run_init_gd_account_link_modal(&ctx, &modal_interaction).await,
		"request-level-modal" => request_level_modal::run_request_level_modal(&ctx, &modal_interaction).await,
		_ => println!("Unreachable")
	};
}
