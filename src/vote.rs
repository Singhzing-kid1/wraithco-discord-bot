use crate::{Context, Error, helper};

use poise::serenity_prelude::{self as serenity, Mentionable};

#[poise::command(slash_command, check = "helper::is_board_of_directors")]
pub async fn start(ctx: Context<'_>, motion: serenity::Channel) -> Result<(), Error> {
    let exsisting_tags = motion
        .clone()
        .guild()
        .map(|c| c.applied_tags.clone())
        .unwrap();

    if !exsisting_tags.contains(
    &std::env::var("MOTION_CLOSED_NO_VOTE_TAG")
        .expect("missing MOTION_CLOSED_NO_VOTE_TAG")
        .parse()?,
    ) || !exsisting_tags.contains(
        &std::env::var("MOTION_CLOSED_ACCEPTED_TAG")
            .expect("missing MOTION_CLOSED_ACCEPTED_TAG")
            .parse()?,
    ) || !exsisting_tags.contains(
        &std::env::var("MOTION_CLOSED_DENIED_TAG")
            .expect("missing MOTION_CLOSED_DENIED_TAG")
            .parse()?,
    ) {

        let poll_builder = serenity::CreatePoll::new()
            .question(format!("{}", motion.mention()))
            .answers(vec![
                serenity::CreatePollAnswer::new().text("Yes"),
                serenity::CreatePollAnswer::new().text("No"),
            ])
            .duration(std::time::Duration::from_hours(32 * 24));

        let message_builder = serenity::CreateMessage::new().poll(poll_builder);
        let message = ctx
            .channel_id()
            .send_message(ctx.http(), message_builder)
            .await?;

        let content = format!(
            "Vote started by {} @ {}",
            ctx.author(),
            ctx.created_at().to_utc()
        );

        let message_builder = serenity::CreateMessage::new().content(content);
        let forum_message = motion
            .id()
            .send_message(ctx.http(), message_builder)
            .await?;

        forum_message.pin(ctx.http()).await?;

        ctx.say(format!("To finish this vote use /vote end {}", message.id))
            .await?;
    } else {
        ctx.say(format!("Cannot start a vote for a closed motion")).await?;
    }

    Ok(())
}

#[poise::command(slash_command, check = "helper::is_board_of_directors")]
pub async fn end(ctx: Context<'_>) -> Result<(), Error> {
    //todo: implement

    Ok(())
}
