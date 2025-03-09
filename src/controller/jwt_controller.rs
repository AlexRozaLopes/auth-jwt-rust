use crate::controller::user_controller::{Login, Mensagem};
use crate::data::users::find_by_nickname_and_password;
use crate::utils::jwt_facade::{create_jwt_for_user, valid_token_jwt};
use actix_web::web::Json;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/valid/token")]
pub async fn valid_token(request: HttpRequest) -> HttpResponse {
    if let Some(token) = request.headers().get("Authorization") {
        if let Ok(jwt) = token.to_str() {
            return valid_token_jwt(jwt.to_string());
        }
    }
    HttpResponse::BadRequest().body("Authorization header ausente ou inválido")
}

#[get("/get/token")]
async fn get_token(request: Json<Login>) -> impl Responder {
    let request = request.into_inner();
    let result = find_by_nickname_and_password(request.nickname, request.password);
    match result {
        Ok(u) => HttpResponse::Ok().json(create_jwt_for_user(u)),
        Err(_) => HttpResponse::Unauthorized().json(Mensagem {
            mensagem: "invalid user".to_string(),
        }),
    }
}
