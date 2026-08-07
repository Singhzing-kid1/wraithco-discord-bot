use crate::{Context, Error, helper};

use poise::serenity_prelude as serenity;


#[poise::command(slash_command, check = "helper::is_board_of_directors")]
pub async fn create(ctx: Context<'_>, #[description = "Title of the motion"] title: String, #[description = "motion details"] body: String) -> Result<(), Error> {
    ctx.say("motion has been created").await?;
    Ok(())
}