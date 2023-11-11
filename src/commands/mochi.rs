use poise::serenity_prelude::Embed;

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

    ctx.channel_id()
        .send_message(&ctx.http(), |m| {
            m.embed(|e| {
                e.colour(0x00ff00)
                    .title(format!("Stats for ", ))
            })
        })
        .await?;

    return Ok(());
}
