use crate::schema::users::dsl::users;
use chrono::{NaiveDateTime, Utc};
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Insertable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub created_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(email: String, password: String, nickname: String) -> User {
        User {
            id: Uuid::new_v4(),
            email,
            password,
            nickname,
            created_at: Option::from(Utc::now().naive_utc()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
    pub nickname: String,
}

pub fn create_user(request: UserRequest) -> User {
    let mut connection = establish_connection();
    let user = User::new(request.email, request.password, request.nickname);
    diesel::insert_into(users::table())
        .values(&user)
        .returning(User::as_returning())
        .get_result::<User>(&mut connection)
        .expect("Error inserting user")
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
