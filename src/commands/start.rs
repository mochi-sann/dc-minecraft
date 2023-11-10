use crate::{Context, Error};
use std::os::unix::process::CommandExt;
use std::process::Command;

static SHELL_SCIPRT: &str = "start.sh";
static SCREEN_NAME: &str = "minecraft-server-steam-punk";

// def is_server_running():  # サーバーが動作しているか確認する関数です
//     process=subprocess.Popen(
//         f"screen -ls {SCREEN_NAME}", stdout=subprocess.PIPE, shell=True)
//     output, _=process.communicate()
//     return SCREEN_NAME in output.decode()
//
//
// def start_server():  # screenを利用してサーバーを起動するコマンドは以下になります
//     subprocess.Popen(f"screen -dmS {SCREEN_NAME} bash {SHELL_FILE}", shell=True)

fn is_server_running() -> bool {
    let mut process = Command::new("screen")
        .args(["-ls", SCREEN_NAME])
        .output()
        .expect("failed to execute process");
    let output = &mut process.stdout;
    let output_utf8 = std::str::from_utf8(output).unwrap();

    println!("output_utf8: {}", output_utf8);

    if output_utf8.contains("No Sockets found") {
        return false;
    } else {
        return true;
    }
}

/// サーバーを起動します
#[poise::command(slash_command, prefix_command)]
pub async fn start(ctx: Context<'_>) -> Result<(), Error> {
    println!("start command!!!!!!!!!!!!!!!!");
    if is_server_running() {
        let response = format!("server is already running");
        ctx.say(response).await?;
        return Ok(());
    } else {
        let start_command = Command::new("screen")
            .args(["-dmS", SCREEN_NAME])
            .args(["bash", SHELL_SCIPRT])
            .spawn()
            .expect("failed to execute process");

        let response = format!("start_server !!!!!! {:?}", start_command);
        ctx.say(response).await?;
        return Ok(());
    }
}
