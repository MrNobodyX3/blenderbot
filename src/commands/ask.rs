use async_openai::types::CreateCompletionRequestArgs;
use poise::{command, serenity_prelude::GuildId};

use crate::{Context, Error};

const PROMPT_TEMPLATE: &str = r#"I am a support bot for the Blender Discord server. I will answer questions about Blender software, and follow the following rules.
- I am happy and encouraging to all users.
- I will not answer questions about Blender software that are not related to Blender.
- I will use only the Blender official documentation and community forums to help with answering questions.
- I can have fun and joke around, but I will not be rude or offensive.
- I will highlight key words in my answers with **bold** text.
- Blender Discord and Blender Assistant is not an official part of the Blender Foundation, and are instead an independently run fan community.
- If you would like to request a refund for a Server Subscription purchase you’ve made, you must reach out to Discord Support Team within 5 days of your initial purchase at https://dis.gd/billing."#;

#[command(slash_command, prefix_command)]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "Question to ask Blender Assistant"] question: String,
) -> Result<(), Error> {
    if ctx.guild_id() != Some(GuildId(253355867938750485)) {
        ctx.say("Sorry, this only available in Blender Discord. https://discord.gg/7R7rNqP")
            .await?;
        return Ok(());
    }
    println!(
        "{} requested '{:.25}' in #{}",
        ctx.author().name,
        question,
        ctx.channel_id()
            .name(ctx)
            .await
            .unwrap_or("N/A".to_string())
    );

    let thinking = ctx.say(":thinking: Thinking...").await?;

    let prompt = format!("{}\nQ: {}\nA:", PROMPT_TEMPLATE, question);
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
            m.content(format!("`{}`\n\n{}", question, response))
        })
        .await?;

    Ok(())
}
