use std::process::Command;

pub static SHELL_SCIPRT: &str = "start.sh";
pub static SCREEN_NAME: &str = "minecraft-server-steam-punk";
pub struct ServerStatus {}

pub fn is_server_running() -> bool {
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

pub fn server_start() {
    Command::new("screen")
        .args(["-dmS", SCREEN_NAME])
        .args(["bash", SHELL_SCIPRT])
        .spawn()
        .expect("failed to execute process");
}

pub fn server_stop() {
    Command::new("screen")
        .args(["-S", SCREEN_NAME])
        .args(["-X", "quit"])
        .spawn()
        .expect("failed to execute process");
}
