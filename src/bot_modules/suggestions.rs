use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::{Context, Error};
use poise::{command, serenity_prelude::{self as serenity, Mentionable}};
use crate::models::{NewSuggestion, Suggestion};

fn create_suggestion(conn: &SqliteConnection, suggestion: &str, author: &serenity::User, message: &serenity::Message) -> Suggestion {
    use crate::schema::suggestions::dsl::*;

    let new_suggestion = NewSuggestion {
        suggestion_text: suggestion,
        suggestion_date: chrono::Utc::now().naive_utc(),
        suggestion_author_id: author.id.0 as i64,
        suggestion_message_id: message.id.0 as i64,
    };

    diesel::insert_into(suggestions)
        .values(&new_suggestion)
        .execute(conn)
        .expect("Error creating suggestion");

    suggestions.order(suggestion_id.desc()).first(conn).unwrap()
}

/// Suggest an idea for the Discord
#[command(slash_command, prefix_command)]
pub async fn suggest(
    ctx: Context<'_>,
    #[description = "What idea to suggest"] #[rest] suggestion: String,
) -> Result<(), Error> {
    let data = ctx.data();
    let conn = data.pool.get().expect("Couldn't get connection from pool");
    let author_id = ctx.author().id.0 as i64;
    let author_name = ctx.author().name.clone();
    let author_icon = ctx.author().avatar_url().unwrap_or_default();

    // get channel with name "suggestions"
    let channel = ctx.guild().unwrap().channels.into_values().find(|c| {
        let guild_channel = c.clone().guild();
        if guild_channel.is_none() {
            return false;
        }

        let guild_channel = guild_channel.unwrap();
        guild_channel.name == "suggestions"
    });
    if channel.is_none() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't find suggestions channel").into());
    }
    let channel = channel.unwrap();
    let message = channel.id().send_message(ctx.discord(), |m| m
        .embed(|e| e
            .title("New suggestion")
            .description(format!("{}\n\nPlease vote on this suggestion using ♥️ and ♠️", suggestion))
            .author(|a| a
                .name(author_name)
                .icon_url(author_icon)
            )
            .timestamp(chrono::Utc::now())
        )
        .reactions(vec![serenity::ReactionType::Unicode("♥️".to_string()), serenity::ReactionType::Unicode("♠️".to_string())])
    ).await?;
    let message_link = message.link_ensured(ctx.discord()).await;
    let _suggestion = create_suggestion(&conn, &suggestion, ctx.author(), &message);
    ctx.send(|cr| cr
        .ephemeral(true)
        .content(format!("Successfully created suggestion!\n\n{}", message_link))
    ).await?;

    Ok(())
}

/// Suggest this message as an idea for the Discord
#[command(context_menu_command = "Suggest this idea")]
pub async fn suggest_message(
    ctx: Context<'_>, message: serenity::Message,
) -> Result<(), Error> {
    let data = ctx.data();
    let conn = data.pool.get().expect("Couldn't get connection from pool");
    let author_id = ctx.author().id.0 as i64;
    let author_name = ctx.author().name.clone();
    let author_icon = ctx.author().avatar_url().unwrap_or_default();
    let suggestion = message.content;

    // get channel with name "suggestions"
    let channel = ctx.guild().unwrap().channels.into_values().find(|c| {
        let guild_channel = c.clone().guild();
        if guild_channel.is_none() {
            return false;
        }

        let guild_channel = guild_channel.unwrap();
        guild_channel.name == "suggestions"
    });
    if channel.is_none() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't find suggestions channel").into());
    }
    let channel = channel.unwrap();
    let message = channel.id().send_message(ctx.discord(), |m| m
        .embed(|e| e
            .title("New suggestion")
            .description(format!("{}\n\nPlease vote on this suggestion using ♥️ and ♠️", suggestion))
            .author(|a| a
                .name(author_name)
                .icon_url(author_icon)
            )
            .timestamp(chrono::Utc::now())
        )
        .reactions(vec![serenity::ReactionType::Unicode("♥️".to_string()), serenity::ReactionType::Unicode("♠️".to_string())])
    ).await?;
    let message_link = message.link_ensured(ctx.discord()).await;
    let _suggestion = create_suggestion(&conn, &suggestion, ctx.author(), &message);
    ctx.send(|cr| cr
        .ephemeral(true)
        .content(format!("Successfully created suggestion!\n\n{}", message_link))
    ).await?;

    Ok(())
}