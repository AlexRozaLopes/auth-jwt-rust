use crate::data::users::User;
use actix_web::HttpResponse;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
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

pub fn valid_token_jwt(mut token: String) -> HttpResponse {
    if token.starts_with("Bearer ") {
        token = token[7..].to_string();
    }

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_ref()).unwrap();

    let claims: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);

    match claims {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::Unauthorized().body("Token inválido ou expirado"),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtToken {
    pub token: String,
}
