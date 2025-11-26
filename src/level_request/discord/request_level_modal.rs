use std::fmt::format;
use log::error;
use serenity::all::{Context, ModalInteraction};
use crate::level_request::model::level_request::{LevelRequest, UpdateLevelRequestMessageId};
use crate::level_request::service::level_request_service::LevelRequestService;
use crate::send_level::model::request_score::RequestRating;
use crate::serenity::discord::{create_thread, invoke_command_ephemeral, invoke_modal_ephemeral, log_action_to_discord, log_error_to_discord, send_level_request_message_to_discord};
use crate::serenity::modals::extract_modal_components;

pub async fn run_request_level_modal(
    ctx: &Context,
    modal_interaction: &ModalInteraction
) {
    let empty_string = String::default();
    let modal_inputs = extract_modal_components(
        &modal_interaction,
        vec![
            "level-id".to_string(),
            "request-rating".to_string(),
            "video-link".to_string(),
            "request-feedback".to_string(),
            "notify".to_string(),
        ]
    );
    let Ok(level_id) = modal_inputs.get("level-id")
        .unwrap_or(&empty_string)
        .parse::<u64>() else {
        return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
    };
    let Ok(request_rating) = modal_inputs.get("request-rating")
        .unwrap_or(&empty_string)
        .parse::<RequestRating>() else {
            return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
        };
    let video_link = modal_inputs.get("video-link")
        .unwrap_or(&empty_string);
    let Ok(has_requested_feedback) = modal_inputs.get("request-feedback")
        .unwrap_or(&empty_string)
        .parse::<bool>() else {
        return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
    };
    let Ok(notify) = modal_inputs.get("notify")
        .unwrap_or(&empty_string)
        .parse::<bool>() else {
        return handle_input_parse_error("Request Feedback", &ctx, &modal_interaction).await;
    };
    let discord_user_id = modal_interaction.user.id.get();
    let level_request_service = LevelRequestService::new();
    let level_request = LevelRequest::new(
        level_id,
        discord_user_id,
        request_rating,
        video_link.to_string(),
        has_requested_feedback,
        notify
    );

    match level_request_service.level_request_service(&level_request).await {
        Ok(requested_level) => {
            match send_level_request_message_to_discord(&ctx, &requested_level).await {
                Ok(message_data) => {
                    if let Err(create_thread_error) =
                        create_thread(&ctx, &modal_interaction.user, message_data.id.get(), &requested_level).await
                    {
                        error!("Error creating thread: {}", create_thread_error);
                        log_error_to_discord(
                            &modal_interaction.user,
                            "creating thread for requested level",
                            &create_thread_error,
                            Some(&requested_level),
                            &ctx
                        )
                            .await;
                    }

                    let update_request_message_id = UpdateLevelRequestMessageId {
                        level_id: requested_level.level_id,
                        discord_message_id: message_data.id.get()
                    };
                    if let Err(error) = level_request_service
                        .update_request_message_id(update_request_message_id)
                        .await
                    {
                        error!("Error updating message ID: {error:?}");
                    }
                }
                Err(send_message_error) => {
                    error!("Error sending message: {send_message_error}");
                }
            }
            let modal_ephemeral_content = "Level has been requested successfully!";
            invoke_modal_ephemeral(&modal_ephemeral_content, &ctx, &modal_interaction).await;

            log_action_to_discord(
                &modal_interaction.user,
                "requested a level",
                Some(&requested_level),
                &ctx
            )
                .await;
        }
        Err(request_level_error) => {
            invoke_modal_ephemeral(&request_level_error.to_string(), &ctx, &modal_interaction).await;
            log_error_to_discord(
                &modal_interaction.user,
                "requesting a level",
                &request_level_error,
                None,
                &ctx
            )
                .await;
        }
    }
}

async fn handle_input_parse_error(
    invalid_field: &str,
    ctx: &Context,
    modal_interaction: &ModalInteraction
) {
    let modal_ephemeral_content= format!("Invalid input for field: {}", invalid_field);
    invoke_modal_ephemeral(modal_ephemeral_content.as_str(), &ctx, &modal_interaction).await
}