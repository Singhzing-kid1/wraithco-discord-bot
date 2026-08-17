use crate::{Context, Error, helper};

use poise::serenity_prelude::{self as serenity, Mentionable};


const CEO_VOTING_POWER: f32 = 0.4;
const COMBINED_BOARD_VOTING_POWER: f32 = 0.6;

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
            .question(format!("{}", motion.id().name(ctx.http()).await.unwrap()))
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

        helper::add_poll(ctx, message.id, motion.id()).await;

        ctx.say(format!("To finish this vote use /vote end {}", message.id))
            .await?;
    } else {
        ctx.say(format!("Cannot start a vote for a closed motion")).await?;
    }

    Ok(())
}

#[poise::command(slash_command, check = "helper::is_board_of_directors")]
pub async fn close(ctx: Context<'_>, message_id: serenity::MessageId) -> Result<(), Error> {
    ctx.channel_id().end_poll(ctx.http(), message_id).await?;

    let board_of_directors_role: serenity::RoleId = std::env::var("BOARD_OF_DIRECTORS").expect("missing BOARD_OF_DIRECTORS").parse()?;
    let ceo_role: serenity::RoleId = std::env::var("CEO").expect("missing CEO").parse()?;

    let guild_id = ctx.guild().unwrap().id;

    let members = guild_id.members(ctx.http(), None, None).await?;
    let count: f32 = members.iter().filter(|m| m.roles.contains(&board_of_directors_role) && !m.roles.contains(&ceo_role)).count() as f32;

    let per_member_voting_power = COMBINED_BOARD_VOTING_POWER / count;

    let mut vote_yes: f32 = 0.0;
    let mut vote_no: f32 = 0.0;

    let message = ctx.channel_id().message(ctx.http(), message_id).await?;
    let poll = message.poll.as_ref().ok_or("not a message with a poll")?;

    for answer in &poll.answers {
        let text = answer.poll_media.text.clone().unwrap();
        let voters = ctx.channel_id().get_poll_answer_voters(ctx.http(), message_id, answer.answer_id, None, None).await?;

        for user in &voters {
            let member = guild_id.member(ctx.http(), user.id).await?;

            match text.as_str() {
                "Yes" => {
                    if member.roles.contains(&ceo_role) {
                        vote_yes += 1.0 * CEO_VOTING_POWER; 
                    } else {
                        vote_yes += 1.0 * per_member_voting_power;
                    }
                },
                "No" => {
                    if member.roles.contains(&ceo_role) {
                        vote_no += 1.0 * CEO_VOTING_POWER;
                    } else {
                        vote_no += 1.0 * per_member_voting_power;
                    }
                },
                _ => {}
            }
        }
    }

    let vote = vote_yes + vote_no;

    let voting_results = format!("{}% voted yes.\n{}% voted no.", vote_yes * 100.0, vote_no * -100.0);

    let motion = helper::remove_and_read_poll(ctx, message_id).await.unwrap();

    if vote > 0.0 {
        let content = format!("{}\n{} has been accepted and closed @ {}", voting_results, motion.mention(), ctx.created_at().to_utc());

        let message_builder = serenity::CreateMessage::new().content(content);

        motion.send_message(ctx.http(), message_builder).await?.pin(ctx.http()).await?;

        let new_tag_id: serenity::ForumTagId = std::env::var("MOTION_CLOSED_ACCEPTED_TAG").expect("missing MOTION_CLOSED_ACCEPTED_TAG").parse()?;

        let edit = serenity::EditThread::new().applied_tags(vec![new_tag_id]);

        motion.edit_thread(ctx.http(), edit).await?;

        ctx.say(format!("{} has been closed and accepted due to voting\n{}", motion.mention(), voting_results)).await?;
    } else {
        let content = format!("{}\n{} has been denied and closed @ {}", voting_results, motion.mention(), ctx.created_at().to_utc());

        let message_builder = serenity::CreateMessage::new().content(content);

        motion.send_message(ctx.http(), message_builder).await?.pin(ctx.http()).await?;

        let new_tag_id: serenity::ForumTagId = std::env::var("MOTION_CLOSED_DENIED_TAG").expect("missing MOTION_CLOSED_DENIED_TAG").parse()?;

        let edit = serenity::EditThread::new().applied_tags(vec![new_tag_id]);

        motion.edit_thread(ctx.http(), edit).await?;

        ctx.say(format!("{} has been closed and denied due to voting\n{}", motion.mention(), voting_results)).await?;
    }


    Ok(())
}
