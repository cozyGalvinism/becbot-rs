use chrono::Utc;
use image::Rgba;
use leptess::LepTess;
use poise::serenity_prelude::{self as serenity, Mentionable};

use rustrict::CensorStr;

use crate::{Data, Error};

async fn scan_image(
    ctx: &serenity::Context,
    attachment: &serenity::Attachment,
    message: &serenity::Message,
    _: &serenity::ChannelId,
    log_channel: &serenity::ChannelId,
) -> serenity::Result<()> {
    let tessdata_dir = std::env::var("TESSDATA").expect("TESSDATA environment variable not set");

    let bytes = attachment.download().await?;
    let leptess = LepTess::new(Some(tessdata_dir.as_str()), "eng");
    if leptess.is_err() {
        return Err(serenity::Error::Other("Failed to initialize leptess"));
    }
    let mut leptess = leptess.unwrap();
    if leptess.set_image_from_mem(&bytes).is_err() {
        return Err(serenity::Error::Other(
            "Unable to set image, it's probably not an image",
        ));
    }

    let img = image::load_from_memory(&bytes).expect("an image");
    let mut img_buf: image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> = img.to_rgba8();
    let mut found_words = Vec::new();
    {
        let boxes =
            leptess.get_component_boxes(leptess::capi::TessPageIteratorLevel_RIL_WORD, true);
        if boxes.is_none() {
            return Ok(());
        }
        let boxes = boxes.unwrap();

        for b in &boxes {
            leptess.set_rectangle(&b);
            let text = leptess.get_utf8_text();
            if text.is_err() {
                continue;
            }
            let text = text.unwrap();
            let is_inappropriate = text.is_inappropriate();
            if is_inappropriate {
                let lp_box: &leptess::capi::Box = b.as_ref();
                let x: i32 = lp_box.x;
                let y: i32 = lp_box.y;
                let w: u32 = lp_box.w.try_into().unwrap();
                let h: u32 = lp_box.h.try_into().unwrap();
                // draw a rectangle around the word
                let red = Rgba([255, 0, 0, 255]);
                img_buf = imageproc::drawing::draw_hollow_rect(
                    &img_buf,
                    imageproc::rect::Rect::at(x - 3, y - 3).of_size(w + 6, h + 6),
                    red,
                );
                found_words.push(text);
            }
        }
    }

    if found_words.is_empty() {
        return Ok(());
    }
    let found_words = found_words.join(", ");
    let found_in = message.channel(ctx).await.unwrap().guild().unwrap();
    log_channel
        .send_message(ctx, |cm| {
            cm.embed(|e| {
                e.title("**Possible blacklisted words in image detected!**")
                    .color(0xFF0000)
                    .timestamp(Utc::now())
                    .author(|a| {
                        let embed_author = a.name(ctx.cache.current_user().name);
                        if ctx.cache.current_user().avatar_url().is_some() {
                            embed_author.icon_url(message.author.avatar_url().unwrap())
                        } else {
                            embed_author
                        }
                    })
                    .field("Author", message.author.mention(), false)
                    .field("Channel", message.channel_id.mention(), false)
                    .field("Author Name", &message.author.name, true)
                    .field("Channel Name", found_in.name, true)
                    .field("Found words", found_words, false)
            })
        })
        .await?;
    img_buf.save("temp.png").unwrap();
    let temp_file = tokio::fs::File::open("temp.png").await?;
    log_channel
        .send_files(ctx, vec![(&temp_file, "processed_image.png")], |m| m)
        .await?;
    tokio::fs::remove_file("temp.png").await?;

    Ok(())
}

pub async fn handle_message(
    ctx: &serenity::Context,
    _: &poise::Framework<Data, Error>,
    message: &serenity::Message,
) -> serenity::Result<()> {
    let log_channel_id: u64 = std::env::var("LOG_CHANNEL_ID").unwrap().parse().unwrap();
    if message.channel_id.0 == log_channel_id {
        return Ok(());
    }
    let log_channel = ctx.http.get_channel(log_channel_id).await?.id();

    let is_inappropriate = message.content.is_inappropriate();
    if is_inappropriate {
        log_channel
            .send_message(ctx, |cm| {
                cm.embed(|e| {
                    e.title("**Blacklisted word detected!**")
                        .color(0xff0000)
                        .timestamp(Utc::now())
                        .author(|a| {
                            let embed_author = a.name(ctx.cache.current_user().name);
                            if ctx.cache.current_user().avatar_url().is_some() {
                                embed_author.icon_url(message.author.avatar_url().unwrap())
                            } else {
                                embed_author
                            }
                        })
                        .field("Author", message.author.mention(), false)
                        .field("Channel", message.channel_id.mention(), false)
                        .field("Message", &message.content, false)
                })
            })
            .await?;
    }

    for attachment in &message.attachments {
        let attachment_clone = attachment.clone();
        let message_clone = message.clone();
        let channel_clone = message.channel_id;
        let context_clone = ctx.clone();
        tokio::spawn(async move {
            let _ = scan_image(
                &context_clone,
                &attachment_clone,
                &message_clone,
                &channel_clone,
                &log_channel,
            )
            .await;
        });
    }

    Ok(())
}
