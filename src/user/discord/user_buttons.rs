use log::error;
use serenity::all::{ComponentInteraction, Context};
use crate::serenity::discord::{invoke_component_ephemeral, log_action_to_discord, log_error_to_discord};
use crate::user::service::user_service::UserService;

pub async fn run_init_gd_account_link_modal(
    ctx: &Context,
    button_interaction: &ComponentInteraction
) {
    let discord_id = button_interaction.user.id.get();
    let user_service = UserService::new();
    let button_ephemeral_content: &str;
    
    match user_service.verify_gd_account_link(discord_id).await {
        Ok(()) => {
            log_action_to_discord(
                &button_interaction.user,
                "verified their GD account link",
                None,
                &ctx
            ).await;
            button_ephemeral_content = "GD account verified successfully";
            invoke_component_ephemeral(button_ephemeral_content, ctx, button_interaction).await;
        }
        Err(verify_gd_account_link_error) => {
            error!("Unable to verify gd account link: {}", verify_gd_account_link_error);
            log_error_to_discord(
                &button_interaction.user,
                "verifying gd account link",
                &verify_gd_account_link_error,
                None,
                &ctx
            ).await;
            button_ephemeral_content = "Unable to verify gd account link.";
            invoke_component_ephemeral(button_ephemeral_content, ctx, button_interaction).await;
        }
    };
}