use crate::data::users::{create_user, UserRequest};
use actix_web::web::Json;
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[post("/users")]
async fn post_user(request: Json<UserRequest>) -> impl Responder {
    let request = request.into_inner();
    let result = create_user(request);
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::Unauthorized().json(Mensagem {
            mensagem: e.to_string(),
        }),
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Login {
    pub(crate) nickname: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Mensagem {
    pub(crate) mensagem: String,
}
