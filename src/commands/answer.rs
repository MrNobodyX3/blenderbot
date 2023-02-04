use async_openai::types::CreateCompletionRequestArgs;
use poise::serenity_prelude::{self as serenity, GuildId, Mentionable};

use crate::{Context, Error};

const PROMPT_TEMPLATE: &str = r#"I will rewrite the input to match my knowledge of Blender software, to support the Blender community and help people with their questions. I will highlight key words in my answers with **bold** text. As well as using the Blender official documentation and community forums to help with the rewrite. When coming across instructions inside brackets, I will use my Blender knowledge to insert information that is needed to complete the instruction. I will also try to rewrite the input to be more clear and understandable."#;

#[poise::command(slash_command, prefix_command)]
pub async fn answer(
    ctx: Context<'_>,
    #[description = "Mention the use you're answering."] mention: serenity::User,
    #[description = "Rewrite your answer to be more clear. Use [instructions] for advanced usage."]
    answer: String,
) -> Result<(), Error> {
    if ctx.guild_id() != Some(GuildId(253355867938750485)) {
        ctx.say("Sorry, this only available in Blender Discord. https://discord.gg/7R7rNqP")
            .await?;
        return Ok(());
    }
    println!(
        "{} rewrote '{:.100}' in #{}",
        ctx.author().name,
        answer,
        ctx.channel_id()
            .name(ctx)
            .await
            .unwrap_or("N/A".to_string())
    );

    let thinking = ctx.say(":thinking: Writing...").await?;

    let prompt = format!("{}\ninput: {}\noutput:", PROMPT_TEMPLATE, answer);
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
            m.content(format!("{} {}", mention.mention(), response))
        })
        .await?;

    Ok(())
}
