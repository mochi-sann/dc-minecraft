use poise::serenity_prelude::{ButtonStyle, Embed};
use serenity::{builder::CreateButton, model::Timestamp};

use crate::{Context, Error};

/// もち
#[poise::command(slash_command, prefix_command)]
pub async fn mochi(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("モチモチﾓﾁﾓﾁﾓ(ﾉ)`ω´(ヾ)");

    let embed = Embed::fake(|e| {
        e.title("Embed title")
            .description("Making a basic embed")
            .field("A field", "Has some content.", false)
    });
    let mut button = CreateButton::default();
    button
        .custom_id("hello world")
        .label("hello world")
        .emoji('\u{2b55}')
        .style(ButtonStyle::Primary);

    // ctx.channel_id()
    //     .send_message(&ctx.http(), |m| {
    //         m.content("モチモチﾓﾁﾓﾁﾓ(ﾉ)`ω´(ヾ)")
    //             .embed(|e| e.title("モチモチﾓﾁﾓﾁﾓ(ﾉ)`ω´(ヾ)"))
    //     })
    //     .await?;
    // ctx.channel_id()
    //     .send_message(&ctx, |m| {
    //         m.content("Please select your favorite animal")
    //             .components(|c| c.create_action_row(|row| row.add_button(button)))
    //     })
    //     .await
    //     .unwrap();
    ctx.say(response).await?;

    return Ok(());
}
