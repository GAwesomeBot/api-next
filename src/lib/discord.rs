extern crate reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiscordOauthTokenResponse {
    pub access_token: String,
}

#[derive(Deserialize, Serialize)]
pub struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
}

pub fn get_oauth_token(access_code: &String) -> Result<DiscordOauthTokenResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let params = [
        ("client_id", ""),
        ("client_secret", ""),
        ("grant_type", "authorization_code"),
        ("code", access_code),
        ("redirect_uri", "http://localhost:8000/login/callback"),
        ("scope", "identify")
    ];
    let response = client.post("https://discordapp.com/api/v6/oauth2/token")
        .form(&params)
        .send()?;
    response.json::<DiscordOauthTokenResponse>()
}

pub fn get_oauth_user(access_token: &String) -> Result<DiscordUser, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    println!("{}", access_token);
    let response = client.get("https://discordapp.com/api/v6/users/@me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?;
    response.json::<DiscordUser>()
}
