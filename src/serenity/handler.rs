use async_trait::async_trait;
use log::{debug, error, info};
use serenity::{
	all::{
		ComponentInteraction, CreateInteractionResponse, GuildId, Interaction, Message,
		MessageType, Ready
	},
	prelude::{Context, EventHandler}
};

use crate::{
	config::{client_config::CLIENT_CONFIG, discord_config::init_verify_message},
	level_request::discord::request_level,
	level_review::discord::review,
	request_manager::discord::request_manager,
	reviewer::discord::reviewer,
	send_level::discord::send_level,
	serenity::modals::get_init_gd_account_link_modal,
	user::discord::user
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, message: Message) {
		if message.kind.eq(&MessageType::ThreadCreated)
			|| message
				.channel_id
				.eq(&CLIENT_CONFIG.discord_public_channel_id)
		{
			if !message
				.author
				.has_role(
					&ctx.http,
					CLIENT_CONFIG.discord_guild_id,
					CLIENT_CONFIG.discord_maintenance_role_id
				)
				.await
				.unwrap()
			{
				if let Err(message_delete_error) = message.delete(&ctx.http).await {
					error!("Unable to delete message: {}", message_delete_error);
				}
			}
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		info!("{} is connected!", ready.user.name);
		let guild_id = GuildId::new(CLIENT_CONFIG.discord_guild_id);
		init_verify_message(&ctx, &ready.user).await;

		guild_id
			.set_commands(
				&ctx.http,
				vec![
					request_level::register_request_level(),
					request_level::register_edit_level_request(),
					request_level::register_delete_level_request(),
					review::register_review(),
					reviewer::register_add_reviewer(),
					reviewer::register_remove_reviewer(),
					send_level::register_send_level(),
					request_manager::register_request_manager(),
					user::register_view_cooldown(),
					user::register_view_user_cooldown(),
				]
			)
			.await
			.expect("Unable to set commands");
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::Command(command) = interaction {
			debug!("Received command interaction: {command:#?}");

			match command.data.name.as_str() {
				"view-cooldown" => user::run_view_cooldown(&ctx, &command).await,
				"view-user-cooldown" => user::run_view_user_cooldown(&ctx, &command).await,
				"request-level" => request_level::run_request_level(&ctx, &command).await,
				"edit-level-request" => request_level::run_edit_level_request(&ctx, &command).await,
				"delete-level-request" => {
					request_level::run_delete_level_request(&ctx, &command).await
				}
				"review" => review::post_level_review(&ctx, &command).await,
				"add-reviewer" => reviewer::run_add_reviewer(&ctx, &command).await,
				"remove-reviewer" => reviewer::run_remove_reviewer(&ctx, &command).await,
				"send-level" => send_level::run_send_level(&ctx, &command).await,
				"request-manager" => request_manager::run_request_manager(&ctx, &command).await,
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
			} else {
			}
		}
	}
}

async fn handle_button_interactions(
	ctx: &Context,
	button_interaction: &ComponentInteraction,
	button_interaction_id: &str
) {
	match button_interaction_id {
		"init-gd-account-link-button" => {
			if let Err(create_modal_error) = button_interaction
				.create_response(
					&ctx.http,
					CreateInteractionResponse::Modal(get_init_gd_account_link_modal())
				)
				.await
			{
				error!(
					"Unable to create init-gd account link modal: {}",
					create_modal_error
				);
			}
		}
		_ => println!("Unreachable")
	};
}

async fn handle_modal_interactions(
	ctx: &Context,
	modal_interaction: &ComponentInteraction,
	modal_interaction_id: &str
) {
	match modal_interaction_id {
		"init-gd-account-link-modal" => {}
		_ => println!("Unreachable")
	};
}
