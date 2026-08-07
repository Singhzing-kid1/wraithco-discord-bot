use crate::{Context, Error, helper};

use poise::serenity_prelude::{self as serenity, Mentionable};

#[poise::command(slash_command, check = "helper::is_board_of_directors")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Title of the motion"] title: String,
    #[description = "motion details"] body: String,
) -> Result<(), Error> {
    let forum_id: serenity::ChannelId = std::env::var("MOTIONS_CHANNEL")
        .expect("missing MOTIONS_CHANNEL")
        .parse()?;

    let content = format!("Author: {} \n Date: {} \n Details: \n {}", ctx.author().mention(), ctx.created_at().to_utc(), body);
    let initial_message = serenity::CreateMessage::new().content(content);


    let builder =
        serenity::CreateForumPost::new(title, initial_message);

    let new_post = forum_id.create_forum_post(ctx.http(), builder).await?;
    
    new_post.id.pin(ctx.http(), serenity::MessageId::new(new_post.id.get())).await?;

    ctx.say(format!("Motion Created: {}", new_post.mention()))
        .await?;

    Ok(())
}
