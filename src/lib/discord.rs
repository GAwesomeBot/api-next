extern crate reqwest;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct DiscordOauthTokenResponse {
    pub access_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: String,
}

#[derive(Deserialize, Serialize)]
pub struct PartialDiscordGuild {
    id: String,
    name: String,
    icon: Option<String>,
}

pub fn get_oauth_token(access_code: &String) -> Result<DiscordOauthTokenResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let params = [
        ("client_id", env::var("CLIENT_ID").unwrap()),
        ("client_secret", env::var("CLIENT_SECRET").unwrap()),
        ("grant_type", "authorization_code".to_string()),
        ("code", access_code.to_string()),
        ("redirect_uri", "http://localhost:3000/login".to_string()),
        ("scope", "identify".to_string())
    ];
    let response = client.post("https://discordapp.com/api/v6/oauth2/token")
        .form(&params)
        .send()?;
    response.json::<DiscordOauthTokenResponse>()
}

pub fn get_oauth_user(access_token: &String) -> Result<DiscordUser, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get("https://discordapp.com/api/v6/users/@me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?;
    response.json::<DiscordUser>()
}

pub fn get_oauth_guilds(access_token: &String) -> Result<Vec<PartialDiscordGuild>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    println!("{}", access_token);
    let response = client.get("https://discordapp.com/api/v6/users/@me/guilds")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?;
    response.json::<Vec<PartialDiscordGuild>>()
}
