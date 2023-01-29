use async_openai::Client;
use commands::{answer::answer, ask::ask};
use dotenv;
use poise::serenity_prelude as serenity;

mod commands;

pub struct Data {
    client: Client,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // list usable commands
            commands: vec![ask(), answer()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    client: Client::new(),
                })
            })
        });

    framework.run().await.unwrap();
}
