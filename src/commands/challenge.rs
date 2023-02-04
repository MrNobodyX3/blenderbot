use async_openai::types::CreateCompletionRequestArgs;
use poise::{command, serenity_prelude::GuildId};

use crate::{Context, Error};

const PROMPT_TEMPLATE: &str = r#"I am a bot in Blender Discord. Using the theme, I provide and give rules for a challenge to create something in Blender software. I am creative and clever in my challenges."#;

#[command(slash_command, prefix_command)]
pub async fn challenge(
    ctx: Context<'_>,
    #[description = "Get a challenge with optional theme"] theme: String,
) -> Result<(), Error> {
    if ctx.guild_id() != Some(GuildId(253355867938750485)) {
        ctx.say("Sorry, this only available in Blender Discord. https://discord.gg/7R7rNqP")
            .await?;
        return Ok(());
    }

    println!(
        "{} challegened '{:.25}' in #{}",
        ctx.author().name,
        theme,
        ctx.channel_id()
            .name(ctx)
            .await
            .unwrap_or("N/A".to_string())
    );

    let thinking = ctx.say(":thinking: Thinking...").await?;

    let prompt = format!("{}\ntheme: {}\nchallenge:", PROMPT_TEMPLATE, theme);
    let req = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .max_tokens(2040u16 - prompt.len() as u16)
        .prompt(prompt)
        .temperature(0.0)
        .top_p(1.0)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .user("async-openai")
        .build()?;

    let response = ctx.data().client.completions().create(req).await?.choices[0]
        .text
        .clone();

    thinking
        .edit(ctx, |m| {
            m.content(format!("Theme: `{}`\n\nChallenge:**{}**", theme, response))
        })
        .await?;

    Ok(())
}
