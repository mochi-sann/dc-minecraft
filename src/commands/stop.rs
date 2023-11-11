use crate::utils::server::{is_server_running, server_stop};
use crate::{Context, Error};

/// サーバーを停止します
#[poise::command(slash_command, prefix_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    println!("start command!!!!!!!!!!!!!!!!");
    if is_server_running() {
        server_stop();
        let response = format!("server stop");
        ctx.say(response).await?;
        return Ok(());
    } else {
        let response = format!("server is stoped");
        ctx.say(response).await?;
        return Ok(());
    }
}
