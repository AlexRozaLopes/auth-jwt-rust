use crate::data::users::{create_user, find_by_nickname_and_password, UserRequest};
use crate::utils::jwt_facade::create_jwt_for_user;
use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[post("/users")]
async fn post_user(request: Json<UserRequest>) -> impl Responder {
    let request = request.into_inner();
    let result = create_user(request);
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::Unauthorized().json(Mensagem{mensagem: e.to_string()}),
    }

}

#[get("/get/token")]
async fn get_token(request: Json<Login>) -> impl Responder {
    let request = request.into_inner();
    let result = find_by_nickname_and_password(request.nickname, request.password);
    match result {
        Ok(u) => HttpResponse::Ok().json(create_jwt_for_user(u)),
        Err(_) => HttpResponse::Unauthorized().json(Mensagem{mensagem: "invalid user".to_string()}),
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct Login {
    nickname: String,
    password: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct Mensagem {
    mensagem: String,
}
