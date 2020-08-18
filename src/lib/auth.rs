use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, Algorithm};

use crate::lib::discord::{DiscordUser};

#[derive(Deserialize, Serialize)]
pub struct ClaimsPayload {
    pub user: DiscordUser,
    pub at: String,
    pub exp: u32,
}

pub fn encode_payload(claims_payload: &ClaimsPayload) -> String {
    encode(&Header::default(), claims_payload, "gab_secret".as_ref()).unwrap()
}

pub fn decode_payload(token: &String) -> ClaimsPayload {
    let token_data = decode::<ClaimsPayload>(token, "gab_secret".as_ref(), &Validation::new(Algorithm::default())).unwrap();
    token_data.claims
}
