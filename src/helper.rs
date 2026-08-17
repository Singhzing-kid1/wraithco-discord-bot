use crate::{Context, Error, commands::motion};

use poise::serenity_prelude::{self as serenity, CreateInteractionResponse::Message};

pub(crate) async fn is_board_of_directors(ctx: Context<'_>) -> Result<bool, Error> {
    let board_of_directors: serenity::RoleId = std::env::var("BOARD_OF_DIRECTORS")
        .expect("missing BOARD_OF_DIRECTORS")
        .parse()?;
    let member = ctx.author();

    let has_role = member
        .has_role(ctx, ctx.guild_id().unwrap(), board_of_directors)
        .await?;

    if !has_role {
        let guild_id = ctx.guild_id().unwrap();
        let roles = guild_id.roles(ctx.http()).await?;

        let role = roles
            .get(&board_of_directors)
            .map(|r| r.name.clone())
            .unwrap();

        let response = format!("you need {} to be able to run this command", role);
        ctx.say(response).await?;
    }

    Ok(has_role)
}


pub(crate) async fn add_poll(ctx: Context<'_>, message_id: serenity::MessageId, motion_id: serenity::ChannelId) {
    ctx.data().active_polls.write().await.insert(message_id, motion_id);
}

pub(crate) async  fn remove_and_read_poll(ctx: Context<'_>, message_id: serenity::MessageId) -> Option<serenity::ChannelId> {
    ctx.data().active_polls.write().await.remove(&message_id)
}
