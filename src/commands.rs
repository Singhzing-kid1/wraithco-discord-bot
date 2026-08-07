use crate::{Context, Error};

use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected User"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn count(
    ctx: Context<'_>,
    #[description = "Role"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    let r = role.as_ref().unwrap();

    let guild_id = ctx.guild().unwrap().id; 

    let members = guild_id.members(ctx.http(), None, None).await?;
    let count = members.iter().filter(|m| m.roles.contains(&r.id)).count();

    let response = format!("{} has {} members", r.name, count);
    ctx.say(response).await?;
    Ok(())
}
