extern crate reqwest;
extern crate serde_json;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::lib::response::{Response, respond};
use crate::lib::discord::{get_oauth_user, get_oauth_token, get_oauth_guilds, PartialDiscordGuild};
use crate::lib::auth::{encode_payload, decode_payload, ClaimsPayload};

#[get("/login")]
pub fn login() -> Redirect {
    Redirect::to("https://discordapp.com/api/oauth2/authorize?client_id=344473886337007617&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Flogin%2Fcallback&response_type=code&scope=identify")
}

#[get("/login/callback?<code>")]
pub fn login_callback(code: String) -> Result<Json<Response<String>>, Box<dyn std::error::Error>> {
    let data = get_oauth_token(&code);
    let access_token = data?.access_token;

    let exp: u32 = 1687481604;

    let user = get_oauth_user(&access_token)?;
    let claims_payload = ClaimsPayload {
        user,
        at: access_token,
        exp,
    };

    let token = encode_payload(&claims_payload);
    Ok(respond::<String>(Some(token), None))
}

#[get("/guilds?<token>")]
pub fn guilds(token: String) -> Result<Json<Response<Vec<PartialDiscordGuild>>>, Box<dyn std::error::Error>> {
    let claims = decode_payload(&token);
    let guilds = get_oauth_guilds(&claims.at)?;
    Ok(respond::<Vec<PartialDiscordGuild>>(Some(guilds), None))
}
