use async_openai::types::CreateCompletionRequestArgs;
use poise::{command, serenity_prelude::GuildId};

use crate::{Context, Error};

#[command(slash_command, prefix_command)]
pub async fn raw(
    ctx: Context<'_>,
    #[description = "Raw GPT-3 input."] input: String,
) -> Result<(), Error> {
    if ctx.guild_id() != Some(GuildId(253355867938750485)) {
        ctx.say("Sorry, this only available in Blender Discord. https://discord.gg/7R7rNqP")
            .await?;
        return Ok(());
    }
    println!(
        "{} requested '{:.25}' in #{}",
        ctx.author().name,
        input,
        ctx.channel_id()
            .name(ctx)
            .await
            .unwrap_or("N/A".to_string())
    );

    let thinking = ctx.say(":thinking: Thinking...").await?;

    let prompt = format!("{}", input);
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
            m.content(format!("`{:.120}...`\n{}", input, response))
        })
        .await?;

    Ok(())
}
