use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;
use poise::send_reply;
use poise::serenity_prelude as serenity;

use crate::diesel::RunQueryDsl;
use crate::models::Quote;
use crate::{models::NewQuote, Context, Error};

fn create_quote(conn: &SqliteConnection, author_id: i64, author: &str, message: &str) {
    use crate::schema::quotes;
    let new_quote = NewQuote {
        message,
        quote_author_id: author_id,
        quote_author: author,
        date: Utc::now().naive_utc(),
    };

    diesel::insert_into(quotes::table)
        .values(&new_quote)
        .execute(conn)
        .expect("Error inserting new quote");
}

fn delete_quote(conn: &SqliteConnection, to_delete: i32) -> bool {
    use crate::schema::quotes::dsl::*;

    // check if quote exists
    let quote = quotes
        .filter(quote_id.eq(to_delete))
        .first::<Quote>(conn)
        .ok();
    if quote.is_none() {
        return false;
    }

    diesel::delete(quotes.filter(quote_id.eq(to_delete)))
        .execute(conn)
        .expect("Error deleting quote");

    quote.is_some()
}

no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

/// Get a quote from the database.
///
/// If `to_get` is None, a random quote will be returned.
/// If `to_get` is Some(i64), the quote with the given ID will be returned.
/// If the quote doesn't exist, None will be returned.
fn get_quote(conn: &SqliteConnection, to_get: Option<i32>) -> Option<Quote> {
    use crate::schema::quotes::dsl::*;

    let query = quotes.into_boxed();

    match to_get {
        Some(id) => query.filter(quote_id.eq(id)).first(conn).ok(),
        None => query.order(RANDOM).first(conn).ok(),
    }
}

/// Add a quote
#[poise::command(
    slash_command,
    context_menu_command = "Add quote",
    required_permissions = "MANAGE_MESSAGES",
    rename = "addquote"
)]
pub async fn add_quote(
    ctx: Context<'_>,
    #[description = "The message to be quoted"] message: serenity::Message,
) -> Result<(), Error> {
    let data = ctx.data();
    let conn = data.pool.get().expect("Couldn't get connection from pool");
    let author = message.author.id;
    let author_name = format!("{}#{}", message.author.name, message.author.discriminator);
    let message_content = message.content;

    create_quote(&conn, author.into(), &author_name, &message_content);

    ctx.send(|f| {
        f.content("Quote added!")
    }).await?;

    Ok(())
}

/// Display a quote
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_MESSAGES",
    guild_cooldown = 5
)]
pub async fn quote(
    ctx: Context<'_>, 
    #[description = "The quote to display"] quote_id: Option<i32>
) -> Result<(), Error> {
    let data = ctx.data();
    let conn = data.pool.get().expect("Couldn't get connection from pool");

    let quote = get_quote(&conn, quote_id);
    if let Some(quote) = quote {
        ctx.say(&format!(
            r#""{}" - {}, {}"#,
            quote.message,
            quote.quote_author,
            quote.date.format("%m/%d/%Y")
        ))
        .await?;
    } else {
        ctx.say("No quote found!").await?;
    }

    Ok(())
}

/// Remove a quote
#[poise::command(
    slash_command,
    required_permissions = "MANAGE_MESSAGES",
    rename = "removequote"
)]
pub async fn remove_quote(
    ctx: Context<'_>,
    #[description = "The quote which should be deleted"] quote_id: i32,
) -> Result<(), Error> {
    let data = ctx.data();
    let conn = data.pool.get().expect("Couldn't get connection from pool");

    let existed = delete_quote(&conn, quote_id);
    if existed {
        ctx.say("Successfully removed quote!").await?;
    } else {
        ctx.say("Couldn't find quote with that ID!").await?;
    }

    Ok(())
}
