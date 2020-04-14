extern crate reqwest;
extern crate serde_json;
use rocket::response::Redirect;
use rocket_contrib::json::Json;

use crate::lib::response::{Response, respond};
use crate::lib::discord::{get_oauth_user, get_oauth_token};
use crate::lib::auth::get_token;

#[get("/login")]
pub fn login() -> Redirect {
    Redirect::to("https://discordapp.com/api/oauth2/authorize?client_id=344473886337007617&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Flogin%2Fcallback&response_type=code&scope=identify")
}

#[get("/login/callback?<code>")]
pub fn login_callback(code: String) -> Result<Json<Response<String>>, Box<dyn std::error::Error>> {
    let data = get_oauth_token(&code);
    let user = get_oauth_user(&data?.access_token);
    let token = get_token(&user?);
    Ok(respond::<String>(Some(token), None))
}
