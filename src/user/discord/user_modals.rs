use crate::serenity::discord::{
	invoke_modal_ephemeral, log_action_to_discord, log_error_to_discord,
};
use crate::serenity::modals::extract_modal_components;
use crate::user::model::discord_user::UserGDAccountLinkRequest;
use crate::user::service::user_service::UserService;
use log::error;
use serenity::all::{Context, ModalInteraction};

pub async fn run_init_gd_account_link_modal(ctx: &Context, modal_interaction: &ModalInteraction) {
	let modal_ephemeral_content: &str;
	let modal_inputs =
		extract_modal_components(&modal_interaction, vec!["gd-username".to_string()]);
	let discord_user_id = modal_interaction.user.id.get();
	let Some(gd_username_input) = modal_inputs.get(&"gd-username".to_string()) else {
		modal_ephemeral_content = "An error occurred while parsing the modal.";
		return invoke_modal_ephemeral(modal_ephemeral_content, &ctx, &modal_interaction).await;
	};
	let user_service = UserService::new();
	let user_gd_account_link_request =
		UserGDAccountLinkRequest::new(discord_user_id, gd_username_input.to_owned());

	match user_service
		.init_gd_account_link(user_gd_account_link_request)
		.await
	{
		Ok(user_gd_account_link) => {
			log_action_to_discord(
				&modal_interaction.user,
				"initiated a GD account link",
				Some(&(
					user_gd_account_link.discord_id,
					user_gd_account_link.gd_player_id,
					user_gd_account_link.gd_username,
				)),
				&ctx,
			)
			.await;
			let formatted_gd_account_link_token_string = format!(
				"Geometry Dash account link initiated! Make a profile post in game \
                that **ONLY** contains the token below. \n\
                *Note*: You will not be able to view this token again, do **NOT** share this token.\
                \n\n {}",
				user_gd_account_link.gd_account_requestx_token
			);

			modal_ephemeral_content = &formatted_gd_account_link_token_string;
			invoke_modal_ephemeral(modal_ephemeral_content, &ctx, &modal_interaction).await;
		}
		Err(init_gd_account_link_error) => {
			error!(
				"Unable to init gd account link: {}",
				init_gd_account_link_error
			);
			log_error_to_discord(
				&modal_interaction.user,
				"initiating GD account link",
				&init_gd_account_link_error,
				Some(gd_username_input),
				&ctx,
			)
			.await;

			modal_ephemeral_content = "Unable to initiate a Geometry Dash account link.";
			invoke_modal_ephemeral(modal_ephemeral_content, &ctx, &modal_interaction).await;
		}
	};
}
