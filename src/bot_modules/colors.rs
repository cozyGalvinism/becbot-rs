use poise::command;
use regex::Regex;

use crate::{Context, Error};

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[a-fA-F0-9]{6}$").unwrap();
}

/// Sets your color
#[command(slash_command, prefix_command)]
pub async fn color(
    ctx: Context<'_>,
    #[description = "The color to use"] #[rest] color: String,
) -> Result<(), Error> {
    let color_u64 = if let Some(stripped) = color.strip_prefix('#') {
        u64::from_str_radix(stripped, 16).unwrap()
    } else {
        u64::from_str_radix(&color, 16).unwrap()
    };

    let author = ctx.author();
    let guild = ctx.guild().unwrap();
    let mut member = guild.member(&ctx.discord(), author.id).await.unwrap();

    for r in member.roles(&ctx.discord()).unwrap() {
        if COLOR_REGEX.is_match(&r.name) {
            guild.delete_role(&ctx.discord(), r.id).await?;
        }
    }

    let mut user_role = guild.role_by_name(&author.id.0.to_string());

    if let Some(user_role) = user_role.as_mut() {
        let user_has_user_role = author.has_role(ctx.discord(), guild.id, user_role.id).await;
        if let Err(e) = user_has_user_role {
            error!("Error while checking if user has role: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error while checking if user has role").into());
        }
        let user_has_user_role = user_has_user_role.unwrap();
        if !user_has_user_role {
            member.add_role(&ctx.discord(), user_role.id).await?;
        }

        user_role.edit(&ctx.discord(), |r| {
            r.colour(color_u64)
        }).await?;
    } else {
        let created_role = guild.create_role(&ctx.discord(), |r| {
            r
                .name(author.id.0.to_string())
                .colour(color_u64)
                .hoist(false)
                .mentionable(false)
        }).await?;
        member.add_role(&ctx.discord(), created_role.id).await?;
    }

    ctx.say(format!("Set your color to {}!", color)).await?;

    Ok(())
}

/// Clears your color
#[command(slash_command, prefix_command)]
pub async fn clearcolor(ctx: Context<'_>) -> Result<(), Error> {
    let author = ctx.author();
    let guild = ctx.guild().unwrap();

    let user_role = guild.role_by_name(&author.id.0.to_string());

    if let Some(user_role) = user_role {
        guild.delete_role(&ctx.discord(), user_role.id).await?;
        ctx.say("Cleared your color!").await?;
    } else {
        ctx.say(format!("You didn't have a color set! Maybe you still have a color from the old bot? In that case, please ask a moderator to remove said role or rename it to your user ID (`{}`).", author.id.0)).await?;
    }

    Ok(())
}