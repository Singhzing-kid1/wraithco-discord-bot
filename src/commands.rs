use crate::{Context, Error, motion};

use poise::serenity_prelude as serenity;

#[poise::command(prefix_command)]
pub async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, subcommands("motion::create"))]
pub async fn motion(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn vote(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
