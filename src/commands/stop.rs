use crate::utils::server::{server_stop, is_server_running};
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
