use std::collections::HashMap;

use chrono::Utc;
use poise::{command, serenity_prelude as serenity};
use rand::Rng;

use crate::{Context, Error};

lazy_static! {
    static ref PRIME_BUBBLE_COMMENTS: Vec<&'static str> = vec![
        "You appear to have terrible luck.",
        "That sucks.",
        "Lower end of the middle, but whatever.",
        "Better than a three, you guess.",
        "Great, we can work with this.",
        "The most favorable result!",
    ];

    static ref FLIPCOIN_RESPONSES: Vec<&'static str> = vec![
        "You lose.",
        "it's /coinflip",
        "GAMBLING? In MY good christian server???"
    ];
}

/// Displays the current time in EST, which is where Happygate lives.
#[command(slash_command)]
pub async fn happytime(ctx: Context<'_>) -> Result<(), Error> {
    // us/eastern time
    let est_now = Utc::now().with_timezone(&chrono_tz::US::Eastern);
    ctx.say(format!(
        "It is currently {} EST",
        est_now.format("%H:%M:%S")
    ))
    .await?;

    Ok(())
}

/// Displays the current time in CST, which is where Lumi lives.
#[command(slash_command)]
pub async fn lumitime(ctx: Context<'_>) -> Result<(), Error> {
    // US/Central
    let cst_now = Utc::now().with_timezone(&chrono_tz::US::Central);
    ctx.say(format!(
        "It is currently {} CST",
        cst_now.format("%H:%M:%S")
    ))
    .await?;

    Ok(())
}

/// Displays the current time in CET, which is where Ven lives.
#[command(slash_command)]
pub async fn ventime(ctx: Context<'_>) -> Result<(), Error> {
    // Europe/Paris
    let cet_now = Utc::now().with_timezone(&chrono_tz::Europe::Paris);
    ctx.say(format!(
        "It is currently {} CET",
        cet_now.format("%H:%M:%S")
    ))
    .await?;

    Ok(())
}

/// Displays the current time in AWST, which is where Teebz lives.
#[command(slash_command)]
pub async fn teebztime(ctx: Context<'_>) -> Result<(), Error> {
    // Australia/Perth
    let awst_now = Utc::now().with_timezone(&chrono_tz::Australia::Perth);
    ctx.say(format!(
        "It is currently {} AWST",
        awst_now.format("%H:%M:%S")
    ))
    .await?;

    Ok(())
}

fn number_to_string(number: i32) -> String {
    match number {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        _ => "nothing".to_string(),
    }
}

fn build_base_embed<'a>(
    ce: &'a mut serenity::CreateEmbed,
    orig: Option<&serenity::Embed>,
    prime_bubble_roll: i32,
) -> &'a mut serenity::CreateEmbed {
    let new_ce = ce
            .title("CATENATIVE DOOMSDAY DICE CASCADER")
            .description(format!("🎲 You press the **PRIME BUBBLE** to allocate the empty **CATENATOR CRUCIBLES** with bubbles. You rolled a **{}**. {} The crucibles are allocated with **{} CASCADER {}**, {}containing a D6. 🎲",
                number_to_string(prime_bubble_roll),
                (&PRIME_BUBBLE_COMMENTS).get(prime_bubble_roll as usize - 1).unwrap(),
                prime_bubble_roll,
                if prime_bubble_roll == 1 { "BUBBLE" } else { "BUBBLES" },
                if prime_bubble_roll > 1 { "each" } else { "" }
            ))
            .image(format!("https://galvinism.ink/cddc_active_{}.gif", prime_bubble_roll));
    if let Some(orig) = orig {
        new_ce.fields(
            orig.fields
                .iter()
                .map(|f| (f.name.clone(), f.value.clone(), f.inline))
                .collect::<Vec<_>>(),
        )
    } else {
        new_ce
    }
}

/// Activate the doomsday device!
#[command(slash_command, aliases("cddc"))]
pub async fn catenativedoomsdaydicecascader(ctx: Context<'_>) -> Result<(), Error> {
    let id = ctx.id();
    let reply = ctx.send(|cr| cr
        .embed(|ce| ce
            .title("CATENATIVE DOOMSDAY DICE CASCADER")
            .description("The doomsday device is looming right in front of you. Do you dare to press the **PRIME BUBBLE**?")
            .image("https://galvinism.ink/cddc_inactive.png")
        )
        .components(|cc| cc
            .create_action_row(|car| car
                .create_button(|cb| cb
                    .style(serenity::ButtonStyle::Success)
                    .label("Press the PRIME BUBBLE!")
                    .custom_id(id)
                    .emoji(serenity::ReactionType::Unicode("🎲".to_string()))
                )
            )
        )
    ).await?.unwrap();

    // the way the Catenative Doomsday Dice Cascader works is:
    // 1. Roll a d6 to determine the amount of dice to roll.
    // 2. You then roll that many dice, with each roll multiplying the number of sides of the next roll.
    // 3. The final roll is the amount of damage.
    let mci = serenity::CollectComponentInteraction::new(ctx.discord())
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(60))
        .filter(move |mci| mci.data.custom_id == id.to_string())
        .collect_limit(1)
        .await;
    if mci.is_none() {
        let mut msg = reply.message().await?;
        msg.edit(ctx.discord(), |m| {
            m.embed(|e| {
                e.title("CATENATIVE DOOMSDAY DICE CASCADER").description(
                    "You failed to press the **PRIME BUBBLE**, the device staying turned off.",
                )
                .image("https://galvinism.ink/cddc_inactive.png")
            })
            .components(|c| c)
        })
        .await?;
        return Ok(());
    }

    let mci = mci.unwrap();
    let mut msg = mci.message.clone();
    let mut dice_sides = 6;
    let prime_bubble_roll = rand::thread_rng().gen_range(1..=6);

    let mut temp_embeds = msg.embeds.clone();
    let mut orig = temp_embeds.get(0);
    msg.edit(ctx.discord(), |m| {
        m
        .embed(|ce| build_base_embed(ce, orig, prime_bubble_roll))
        .components(|c| c)
    })
    .await?;

    mci.create_interaction_response(ctx.discord(), |ir| ir
        .kind(serenity::InteractionResponseType::DeferredUpdateMessage)
    )
    .await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    for i in 0..prime_bubble_roll - 1 {
        let cascader_result = rand::thread_rng().gen_range(1..=dice_sides);
        let old_sides = dice_sides;
        dice_sides *= cascader_result;
        let number = i + 1;
        if i == prime_bubble_roll - 2 {
            msg.edit(ctx.discord(), |m| {
                m.embed(|ce| {
                    build_base_embed(ce, orig, prime_bubble_roll).field(
                        format!(
                            "🎲 **{} CASCADER**",
                            inflector::numbers::ordinalize::ordinalize(number.to_string().as_str()).to_uppercase()
                        ),
                        format!(
                            "The **{}** cascader is pressed for a resulting multiplier of **{}**. The final remaining die now has **{}** * **{}** = **{}** sides!",
                            inflector::numbers::ordinalize::ordinalize(number.to_string().as_str()),
                            cascader_result,
                            old_sides,
                            cascader_result,
                            dice_sides
                        ),
                        false,
                    )
                })
            })
            .await?;
        } else {
            msg.edit(ctx.discord(), |m| {
                m.embed(|ce| {
                    build_base_embed(ce, orig, prime_bubble_roll).field(
                        format!(
                            "🎲 **{} CASCADER**",
                            inflector::numbers::ordinalize::ordinalize(number.to_string().as_str()).to_uppercase()
                        ),
                        format!(
                            "The **{}** cascader is pressed for a resulting multiplier of **{}**. The remaining {} dice now have **{}** * **{}** = **{}** sides each.",
                            inflector::numbers::ordinalize::ordinalize(number.to_string().as_str()),
                            cascader_result,
                            prime_bubble_roll - number,
                            old_sides,
                            cascader_result,
                            dice_sides
                        ),
                        false,
                    )
                })
            })
            .await?;
        }
        temp_embeds = msg.embeds.clone();
        orig = temp_embeds.get(0);
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
    let result = rand::thread_rng().gen_range(1..=dice_sides);
    msg.edit(ctx.discord(), |m| {
        m.embed(|ce| {
            build_base_embed(ce, orig, prime_bubble_roll).field(
                "🎲 **FINAL CASCADER**",
                format!(
                    "You roll the last die by pressing the final **DOOMSDAY CASCADER**. This will trigger the weapon on the terrible device and potentially deal up to **{} HIT POINTS IN DAMAGE**!",
                    dice_sides
                ),
                false,
            )
        })
    }).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    msg.edit(ctx.discord(), |m| {
        m.embed(|ce| {
            build_base_embed(ce, orig, prime_bubble_roll).field(
                "🎲 **FINAL CASCADER**",
                format!(
                    "The **DOOMSDAY CASCADER** rolls a {}!",
                    result
                ),
                false,
            )
            .image("https://galvinism.ink/cddc_inactive.png")
        })
    }).await?;
    if result == 1 {
        ctx.say("https://www.homestuck.com/images/extras/ps000020_9.gif").await?;
    }

    Ok(())
}

/// hydration check
#[command(slash_command)]
pub async fn hydrate(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("don't forget to drink water").await?;

    Ok(())
}

/// That's not very family friendly of you.
#[command(slash_command)]
pub async fn familyfriendly(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://galvinism.ink/nff.jpg").await?;

    Ok(())
}

/// Hug someone
#[command(slash_command, context_menu_command = "Give this person a hug")]
pub async fn hug(
    ctx: Context<'_>,
    #[description = "The person to hug"] user: serenity::User,
) -> Result<(), Error> {
    if let Some(nick) = user.nick_in(ctx.discord(), ctx.guild().unwrap()).await {
        ctx.say(format!("*hugs {}*", nick)).await?;
    } else {
        ctx.say(format!("*hugs {}*", user.name)).await?;
    }

    Ok(())
}

/// Say hello!
#[command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello!").await?;

    Ok(())
}

/// Ping pong
#[command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;

    Ok(())
}

/// you know what this does
#[command(slash_command)]
pub async fn flipcoin(ctx: Context<'_>) -> Result<(), Error> {
    let result = rand::thread_rng().gen_range(0..FLIPCOIN_RESPONSES.len());
    ctx.say(FLIPCOIN_RESPONSES[result]).await?;

    Ok(())
}

/// you know what this does, too
#[command(slash_command)]
pub async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("it's !flipcoin").await?;

    Ok(())
}

/// I need healing
#[command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://www.youtube.com/watch?v=yD2FSwTy2lw").await?;

    Ok(())
}

/// Post the link to the ban appeal form
#[command(slash_command)]
pub async fn banappeal(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://docs.google.com/forms/d/e/1FAIpQLScfna7CI_XMEX-szOBC7h_E1XJDSNCjYEYBId69QwuZnITOCw/viewform").await?;

    Ok(())
}

/// no
#[command(slash_command)]
pub async fn addjohn(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("no").await?;

    Ok(())
}

/// do u kno da wae
#[command(slash_command)]
pub async fn ohno(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://media.discordapp.net/attachments/431932541612589077/544587477508161547/Ilikestevenuniversealotbutwhydoi_ad1871fcdc058915b00d12d5968a0d0a.png").await?;

    Ok(())
}

/// Post the link to the radio
#[command(slash_command)]
pub async fn radio(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://www.youtube.com/luminantAegis/live").await?;

    Ok(())
}

/// No roleplaying in my server
#[command(slash_command)]
pub async fn rp(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("We all know role playing isn't allowed here, so you must be looking for this instead! https://mspfa.com/?s=25171&p=1").await?;

    Ok(())
}

/// Send someone off
#[command(slash_command)]
pub async fn copper(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(r#"Copper is "Cu" on the periodic table. When read it sounds like "see you", a shortened version of the phrase "See you later", which is commonly used as a sendoff to someone."#).await?;

    Ok(())
}