use crate::{Context, Error, motion, vote};

use poise::serenity_prelude as serenity;

#[poise::command(prefix_command)]
pub async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[poise::command(slash_command, subcommands("motion::create", "motion::close"))]
pub async fn motion(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, subcommands("vote::start", "vote::close"))]
pub async fn vote(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
