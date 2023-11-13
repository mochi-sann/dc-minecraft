pub mod commands;
pub mod utils;
use commands::{age::age, mochi::mochi, ping::ping, start::start, stop::stop, status::status};
use std::env;

use dotenv::dotenv;

use poise::serenity_prelude as serenity;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), ping(), start(), stop(), mochi(), status()],
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {})
            })
        });

    framework.run().await.unwrap();

    // Set gateway intents, which decides what events the bot will be notified about
}
