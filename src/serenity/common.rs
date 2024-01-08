use serenity::all::{
	CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage
};

pub async fn invoke_ephermal(content: &str, ctx: &Context, command: &CommandInteraction) {
	let data = CreateInteractionResponseMessage::new()
		.ephemeral(true)
		.content(content);
	let builder = CreateInteractionResponse::Message(data);
	if let Err(err) = command.create_response(&ctx.http, builder).await {
		println!("Cannot respond to slash command: {err}");
	}
}
