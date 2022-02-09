use chrono::Utc;
use diesel::prelude::*;
use diesel::SqliteConnection;
use poise::{command, serenity_prelude as serenity};

use crate::{models::NewCan, Context, Error};

fn add_can(conn: &SqliteConnection, author: &serenity::User) -> i64 {
    use crate::schema::cans;
    let username = format!("{}#{}", author.name, author.discriminator);

    let new_can = NewCan {
        user_id: author.id.0 as i64,
        user: username.as_str(),
        date: Utc::now().naive_utc(),
    };

    diesel::insert_into(cans::table)
        .values(&new_can)
        .execute(conn)
        .expect("Error inserting new can");

    cans::table.count().get_result(conn).unwrap()
}

/// Add a can on lumiDiscord
#[command(slash_command, guild_cooldown = 35, prefix_command, aliases("addbear", "asscan"))]
pub async fn addcan(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();
    let data = ctx.data();
    let conn = data.pool.get().expect("Couldn't get connection from pool");

    let new_count = add_can(&conn, author);
    ctx.say(format!("You place a can on lumiDiscord. There's now {} cans.  Someone can add another in 35 seconds.", new_count)).await?;

    Ok(())
}
