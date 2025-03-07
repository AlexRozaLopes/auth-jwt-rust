use crate::data::users::User;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;

pub fn create_jwt_for_user(user: User) -> JwtToken {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_ref()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user_id", user.id.to_string());
    claims.insert("email", user.email.to_string());
    claims.insert("nickname", user.nickname.to_string());
    let jwt = claims.sign_with_key(&key).unwrap();
    JwtToken { token: jwt }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtToken {
    pub token: String,
}
