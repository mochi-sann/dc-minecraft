use crate::utils::server::{is_server_running, server_start};
use crate::{Context, Error};

/// サーバーを起動します
#[poise::command(slash_command, prefix_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    println!("start command!!!!!!!!!!!!!!!!");
    if is_server_running() {
        let response = format!("server is already running");
        ctx.say(response).await?;
        return Ok(());
    } else {
        server_start();

        let response = format!("start_server !!!!!! ");
        ctx.say(response).await?;
        return Ok(());
    }
}
