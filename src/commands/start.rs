use crate::{Context, Error};
use std::process::Command;

static SHELL_SCIPRT: &str = "start.sh";
static SCREEN_NAME: &str = "minecraft-server-steam-punk";

/// サーバーを起動します
#[poise::command(slash_command, prefix_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    println!("start command!!!!!!!!!!!!!!!!");

    let run_server_command = Command::new("screen")
        .args(["-s", SCREEN_NAME])
        .args(["bash", SHELL_SCIPRT])
        .output()
        .expect("failed to execute process");


    let response = format!("s account was created at  {:?}", run_server_command);
    ctx.say(response).await?;
    Ok(())
}
