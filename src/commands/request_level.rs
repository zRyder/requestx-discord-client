use crate::{
	config::requestx_api_config::REQUESTX_API_CONFIG,
	model::serenity::request_rating_choice::RequestRatingChoice, Data
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn request_level(
	ctx: Context<'_>,
	#[description = "Level ID of the level to request"] level_id: u64,
	#[description = "Requested rating of the level request"] requested_score: RequestRatingChoice,
	#[description = "Video of the level being requested"] video_link: Option<String>
) -> Result<(), Error> {
	let test_response = format!(
		"This user {} requested level: {} and requested {:?} with vid: {} <test:1193182838514794506>",
		ctx.author().id,
		level_id,
		requested_score,
		if let Some(link) = video_link {
			link
		} else {
			"No link provided".to_string()
		},
	);
	match ctx.say(test_response).await {
		Ok(_) => Ok(()),
		Err(error) => Err(Error::try_from(error).unwrap())
	}
}
