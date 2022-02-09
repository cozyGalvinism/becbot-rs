use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::{Data, Error};

pub async fn handle_message(ctx: &serenity::Context, framework: &poise::Framework<Data, Error>, message: &serenity::Message) -> serenity::Result<()> {
    if message.author.bot {
        return Ok(());
    }

    if message.guild_id.is_none() {
        return Ok(());
    }

    if message.content == "wkjfneasdf" {
        message.reply_ping(ctx, "That's not a word, but rather an oddly specific keyboard mash. Good job!").await?;
        return Ok(());
    }

    // test == turing
    if message.content == "test" {
        message.reply_ping(ctx, "turing").await?;
        return Ok(());
    }

    // F
    if message.content.to_lowercase().starts_with('f')  {
        message.reply_ping(ctx, ":regional_indicator_f:").await?;
        return Ok(());
    }

    // ^^
    if message.content.starts_with("^^ ") || message.content == "^^" {
        message.reply_ping(ctx, "^u^").await?;
        return Ok(());
    }

    // \o/
    if message.content.starts_with("\\o/") {
        message.reply_ping(ctx, "https://imgur.com/qJYq4Xn").await?;
        return Ok(());
    }

    // OBAMA IS GONE
    if message.content.to_lowercase().starts_with("crab") {
        message.reply_ping(ctx, "https://tenor.com/view/crab-safe-dance-gif-13211112").await?;
        return Ok(());
    }

    // sock ruse
    if message.content.to_lowercase().starts_with("sock ruse")  {
        message.reply_ping(ctx, "it was a DISTACTION").await?;
        return Ok(());
    }

    // Pizza time
    if message.content.to_lowercase().starts_with("what time is it?") {
        message.reply_ping(ctx, "https://tenor.com/view/pizza-time-its-delivery-gif-13167414").await?;
        return Ok(());
    }

    if message.content.to_lowercase().contains("radio lags") || message.content.to_lowercase().contains("radio is lagging") {
        let lumi_user = ctx.http.get_user(84774207140945920).await.ok();
        let mention: String = if let Some(user) = lumi_user {
            user.mention().to_string()
        } else {
            "<@84774207140945920>".to_string()
        };

        message.reply_ping(ctx, mention).await?;
        message.reply_ping(ctx, "https://media.discordapp.net/attachments/551868267099193374/768520702402887710/unknown.png").await?;
        return Ok(());
    }

    // Mod only responses
    let roles = ctx.http.get_guild_roles(message.guild_id.unwrap().0).await?;
    let mod_role = roles.iter().find(|r| r.name == "Moderator");
    if let Some(mod_role) = mod_role {
        let has_role = message.author.has_role(ctx, message.guild_id.unwrap(), mod_role).await?;
        if has_role {
            if message.content.starts_with("chirp") {
                message.reply_ping(ctx, "chirp chirp!").await?;
                return Ok(());
            }

            if message.content.starts_with("bark") {
                message.reply_ping(ctx, "bork").await?;
                return Ok(());
            }

            if message.content.starts_with("bear") {
                message.reply_ping(ctx, ":bear:").await?;
                return Ok(());
            }
        }
    }

    Ok(())
}