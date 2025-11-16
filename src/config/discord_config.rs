use std::process;
use log::error;
use serenity::all::{ButtonStyle, ChannelId, Context, CreateMessage, GetMessages, Message, User};
use serenity::builder::CreateButton;
use crate::config::client_config::CLIENT_CONFIG;
use crate::serenity::discord::{get_init_gd_account_link_embed, get_verify_gd_account_link_embed};

pub async fn init_verify_message(
    ctx: &Context,
    bot_user: &User
) {
    let verify_channel = ChannelId::new(CLIENT_CONFIG.discord_verify_channel_id);

    let messages = verify_channel.messages(
        &ctx.http,
        GetMessages::new()
    ).await
        .map_err(|read_messages_error| {
            error!("Failed to retrieve messages from verification channel: {}", read_messages_error);
            process::exit(1)
        })
        .ok()
        .unwrap_or_default();

    if !messages.is_empty() {
        let (init_message_exists, verify_message_exists) = messages.iter()
            .fold((false, false), |(init_exists, verify_exists), message| {
                (
                    init_exists || check_message(&message, "init".to_string()),
                    verify_exists || check_message(&message, "verify".to_string())
                )
            });

        if !init_message_exists {
            send_init_message(&ctx, &bot_user, &verify_channel).await
        }
        if !verify_message_exists {
            send_verify_message(&ctx, &bot_user, &verify_channel).await
        }
    } else {
        send_init_message(&ctx, &bot_user, &verify_channel).await;
        send_verify_message(&ctx, &bot_user, &verify_channel).await;
    }
}

fn check_message(message: &Message, condition_string: String) -> bool {
    if let Some(embed) = message.embeds.get(0) {
        if let Some(footer) = &embed.footer {
            return footer.text == condition_string
        }
    }

    false
}

async fn send_init_message(
    ctx: &Context,
    bot_user: &User,
    verify_channel: &ChannelId
) {
    let init_message = CreateMessage::new()
        .button(
            CreateButton::new("init-gd-account-link-button")
                .label("Link GD Account")
                .style(ButtonStyle::Primary)
        )
        .embed(get_init_gd_account_link_embed(&bot_user));

    verify_channel.send_message(&ctx.http, init_message)
        .await
        .map_err(|send_init_message_error| {
            error!(
                            "Failed to send init message to verification channel: {}",
                            send_init_message_error
                        );
        })
        .ok();
}

async fn send_verify_message(
    ctx: &Context,
    bot_user: &User,
    verify_channel: &ChannelId
) {
    let verify_message = CreateMessage::new()
        .button(
            CreateButton::new("verify-gd-account-link-button")
                .label("Verify GD Account Link")
                .style(ButtonStyle::Primary)
        )
        .embed(get_verify_gd_account_link_embed(&bot_user));

    verify_channel.send_message(&ctx.http, verify_message)
        .await
        .map_err(|send_verify_message_error| {
            error!(
                            "Failed to send verify message to verification channel: {}",
                            send_verify_message_error
                        );
        })
        .ok();
}