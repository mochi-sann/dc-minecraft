use crate::{Context, Error};
use poise::serenity_prelude::Embed;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRoot {
    pub online: bool,
    pub host: String,
    pub port: i64,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    #[serde(rename = "eula_blocked")]
    pub eula_blocked: bool,
    #[serde(rename = "retrieved_at")]
    pub retrieved_at: i64,
    #[serde(rename = "expires_at")]
    pub expires_at: i64,
    pub version: Version,
    pub players: Players,
    pub motd: Motd,
    pub icon: String,
    pub mods: Vec<Value>,
    pub software: Value,
    pub plugins: Vec<Value>,
    #[serde(rename = "srv_record")]
    pub srv_record: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename = "name_raw")]
    pub name_raw: String,
    #[serde(rename = "name_clean")]
    pub name_clean: String,
    #[serde(rename = "name_html")]
    pub name_html: String,
    pub protocol: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Players {
    pub online: i64,
    pub max: i64,
    pub list: Vec<List>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub uuid: String,
    #[serde(rename = "name_raw")]
    pub name_raw: String,
    #[serde(rename = "name_clean")]
    pub name_clean: String,
    #[serde(rename = "name_html")]
    pub name_html: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Motd {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

///サーバーの状態を表示
#[poise::command(slash_command, prefix_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let url = "https://api.mcstatus.io/v2/status/java/mc.mochi33.com";
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.json::<JsonRoot>().await?;
    // let response = format!("pong hello");
    let mut response = format!("Server is ");

    if body.online {
        response += "online\n";
    } else {
        response += "offline\n";
    }
    let embed = Embed::fake(|e| e.title("Server status").description(response.to_string()));

    body.players.list.iter().for_each(|player| {
        response += &format!("{} ", player.name_raw);
    });
    ctx.channel_id()
        .send_message(&ctx.http(), |m| {
            m.content(response.to_string())
                .embed(|e| e.title("Server status").description(response.to_string()))
        })
        .await?;
    ctx.say(response).await?;
    Ok(())
}
