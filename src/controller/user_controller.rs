use actix_web::{post, HttpResponse, Responder};
use actix_web::web::Json;
use crate::data::users::{create_user, UserRequest};
use crate::utils::jwt_facade::create_jwt_for_user;

#[post("/users")]
async fn post_user(request: Json<UserRequest>) -> impl Responder {
    let request = request.into_inner();
    let user = create_user(request);
    println!("{:?}", user);
    HttpResponse::Ok().json(create_jwt_for_user(user))
}