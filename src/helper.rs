use crate::{Context, Error};

use poise::serenity_prelude as serenity;

pub(crate) async fn is_board_of_directors(ctx: Context<'_>) -> Result<bool, Error> {
    let board_of_directors: serenity::RoleId = std::env::var("BOARD_OF_DIRECTORS").expect("missing BOARD_OF_DIRECTORS").parse()?;
    let member = ctx.author();

    let has_role = member.has_role(ctx, ctx.guild_id().unwrap(), board_of_directors).await?;

    Ok(has_role)
}