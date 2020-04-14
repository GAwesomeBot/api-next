use jsonwebtoken::{encode, Header};

use crate::lib::discord::DiscordUser;

pub fn get_token(user: &DiscordUser) -> String {
    encode(&Header::default(), user, "gab_secret".as_ref()).unwrap()
}
