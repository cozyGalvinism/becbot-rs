#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod bot_modules;
mod models;
mod schema;

use std::env;

use diesel::{r2d2::ConnectionManager, SqliteConnection, connection::SimpleConnection};
use dotenv::dotenv;
use poise::{serenity_prelude::{self as serenity, GatewayIntents}, PrefixFrameworkOptions};

pub struct UserData {
    pub pool: r2d2::Pool<ConnectionManager<SqliteConnection>>,
}

type Data = UserData;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, UserData, Error>;

async fn register_all_commands(guild_id: u64, ctx: &serenity::Context, framework: &poise::FrameworkContext<'_, Data, Error>) -> Result<usize, serenity::Error> {
    let mut commands_builder = serenity::CreateApplicationCommands::default();
    let commands = &framework.options().commands;
    for command in commands {
        if let Some(slash_command) = command.create_as_slash_command() {
            commands_builder.add_application_command(slash_command);
        }
        if let Some(context_menu_command) = command.create_as_context_menu_command() {
            commands_builder.add_application_command(context_menu_command);
        }
    }
    let commands_builder = serenity::json::Value::Array(commands_builder.0);
    let registered = ctx.http.create_guild_application_commands(guild_id, &commands_builder).await?;
    Ok(registered.len())
}

async fn on_event(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot: _ } => {
            info!("Becbot is starting up, caching guilds...");
        },
        poise::Event::CacheReady { guilds} => {
            info!("Cache ready, registering commands...");
            for g in guilds {
                let registered_amount = register_all_commands(g.0, ctx, &framework).await?;
                info!("Registered {} commands for guild {}", registered_amount, g.0);
            }
            info!("Commands registered! Have fun!");
            ctx.set_activity(serenity::Activity::playing(format!("Becbot Reloaded v{}", env!("CARGO_PKG_VERSION")))).await;
        },
        poise::Event::Message {new_message} => {
            let _ = bot_modules::autoresponder::handle_message(ctx, &framework, new_message).await;
            
            let log_channel_id = std::env::var("LOG_CHANNEL_ID");
            if log_channel_id.is_err() {
                warn!("LOG_CHANNEL_ID not set! Moderation module disabled...");
                return Ok(());
            }
        },
        poise::Event::ReactionAdd { add_reaction } => {
            bot_modules::suggestions::handle_reaction_add(ctx, framework, add_reaction).await?;
        },
        _ => (),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .with_module_level("serenity", log::LevelFilter::Off)
        .with_module_level("tracing", log::LevelFilter::Off)
        .init().ok();
    
    info!("Booting Becbot v{}", env!("CARGO_PKG_VERSION"));

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let database_url =
        env::var("DATABASE_URL").expect("Expected a database url in the environment");

    let framework = poise::Framework::builder()
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .token(token)
        .user_data_setup(
            move |_ctx, _ready, _framework: &poise::Framework<UserData, Error>| {
                Box::pin(async move {
                    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
                    let pool = r2d2::Pool::builder()
                        .build(manager)
                        .expect("Failed to create pool.");
                    
                    // run some pragmas
                    let conn = pool.get().expect("Couldn't get connection from pool");
                    conn.batch_execute("
                        PRAGMA journal_mode = WAL;          -- better write-concurrency
                        PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
                        PRAGMA wal_autocheckpoint = 1000;   -- write WAL changes back every 1000 pages, for an in average 1MB WAL file. May affect readers if number is increased
                        PRAGMA wal_checkpoint(TRUNCATE);    -- free some space by truncating possibly massive WAL files from the last run.
                        PRAGMA busy_timeout = 5000;         -- sleep if the database is busy
                        PRAGMA foreign_keys = ON;           -- enforce foreign keys
                    ").unwrap();
                    
                    Ok(UserData { pool })
                })
            },
        )
        .options(poise::FrameworkOptions {
            listener: |ctx, event, framework, user_data| {
                Box::pin(on_event(ctx, event, framework, user_data))
            },
            commands: vec![
                bot_modules::quotes::add_quote(),
                bot_modules::quotes::remove_quote(),
                bot_modules::quotes::quote(),
                bot_modules::misc::catenativedoomsdaydicecascader(),
                bot_modules::misc::teebztime(),
                bot_modules::misc::lumitime(),
                bot_modules::misc::ventime(),
                bot_modules::misc::happytime(),
                bot_modules::misc::hydrate(),
                bot_modules::misc::hug(),
                bot_modules::misc::hug_someone(),
                bot_modules::misc::familyfriendly(),
                bot_modules::misc::hello(),
                bot_modules::misc::ping(),
                bot_modules::misc::flipcoin(),
                bot_modules::misc::help(),
                bot_modules::misc::coinflip(),
                bot_modules::misc::addjohn(),
                bot_modules::misc::ohno(),
                bot_modules::misc::radio(),
                bot_modules::misc::rp(),
                bot_modules::misc::copper(),
                bot_modules::misc::worldbuilding(),
                bot_modules::colors::color(),
                bot_modules::colors::clearcolor(),
                bot_modules::suggestions::suggest(),
                bot_modules::suggestions::suggest_message()
            ],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .build().await.unwrap();
    
    let frm = framework.clone();
    tokio::spawn(async move {
        let shard_manager = frm.shard_manager();
        tokio::signal::ctrl_c().await.expect("Failed to install CTRL+C handler");
        println!("CTRL+C received, shutting down...");
        shard_manager.lock().await.shutdown_all().await;
    });

    framework.start().await.unwrap();
}

#[cfg(test)]
mod tests {
    use break_eternity::Decimal;

    #[test]
    fn test_highest_cnnc() {
        let mut dice_sides: Decimal = 6.into();

        // first roll
        dice_sides = dice_sides * dice_sides;
        // second roll
        dice_sides = dice_sides * dice_sides;
        // third roll
        dice_sides = dice_sides * dice_sides;
        // fourth roll
        dice_sides = dice_sides * dice_sides;
        // fifth roll
        dice_sides = dice_sides * dice_sides;
        // sixth roll
        dice_sides = dice_sides * dice_sides;

        println!("{}", dice_sides);
    }
}