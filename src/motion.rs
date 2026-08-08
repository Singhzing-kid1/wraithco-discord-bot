use std::os::unix::process::parent_id;

use crate::{Context, Error, helper};

use poise::serenity_prelude::{self as serenity, Mentionable};

#[poise::command(slash_command, check = "helper::is_board_of_directors")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Title of the motion"] title: String,
    #[description = "motion details"] details: String,
) -> Result<(), Error> {
    let forum_id: serenity::ChannelId = std::env::var("MOTIONS_CHANNEL")
        .expect("missing MOTIONS_CHANNEL")
        .parse()?;

    let content = format!(
        "Author: {}\nDate: {}\nDetails:\n {}",
        ctx.author().mention(),
        ctx.created_at().to_utc(),
        details
    );
    let initial_message = serenity::CreateMessage::new().content(content);

    let channel = forum_id.to_channel(ctx.http()).await?.guild().unwrap();
    let tag_id: serenity::ForumTagId = std::env::var("MOTION_OPEN_TAG").expect("missing MOTION_OPEN_TAG").parse()?;

    let builder = serenity::CreateForumPost::new(title, initial_message).set_applied_tags(vec![tag_id]);

    let new_post = forum_id.create_forum_post(ctx.http(), builder).await?;

    new_post
        .id
        .pin(ctx.http(), serenity::MessageId::new(new_post.id.get()))
        .await?;

    ctx.say(format!("Motion Created: {}", new_post.mention()))
        .await?;

    Ok(())
}

#[poise::command(slash_command, check = "helper::is_board_of_directors")]
pub async fn close(ctx: Context<'_>, motion: serenity::Channel) -> Result<(), Error> {
    let content = format!(
        "the motion ({}) has been closed @ {} by {} before voting.",
        motion.mention(),
        ctx.created_at().to_utc(),
        ctx.author().mention()
    );

    let message_builder = serenity::CreateMessage::new().content(content);

    let message = motion
        .id()
        .send_message(ctx.http(), message_builder)
        .await?;

    message.pin(ctx.http()).await?;



    let new_tag_id: serenity::ForumTagId = std::env::var("MOTION_CLOSED_NO_VOTE_TAG").expect("missing MOTION_CLOSED_NO_VOTE_TAG").parse()?;

    let edit = serenity::EditThread::new().applied_tags(vec![new_tag_id]);

    motion.id().edit_thread(ctx.http(), edit).await?;

    ctx.say(format!("{} has been closed.", motion.mention()))
        .await?;

    Ok(())
}
