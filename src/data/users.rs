use crate::schema::users::dsl::users;
use crate::schema::users::{nickname, password};
use chrono::{NaiveDateTime, Utc};
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
    pub fn new(email: String, pass: String, nick: String) -> User {
        User {
            id: Uuid::new_v4(),
            email,
            password: pass,
            nickname: nick,
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

pub fn create_user(request: UserRequest) -> QueryResult<User> {
    let mut connection = establish_connection();
    let mut sha256 = Sha256::new();
    sha256.update(request.password.as_bytes());
    let hash = format!("{:x}", sha256.finalize());
    let user = User::new(request.email, hash, request.nickname);
    diesel::insert_into(users::table())
        .values(&user)
        .returning(User::as_returning())
        .get_result(&mut connection)
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn find_by_nickname_and_password(nick: String, pass: String) -> QueryResult<User> {
    let mut connection = establish_connection();
    let mut sha256 = Sha256::new();
    sha256.update(pass.as_bytes());
    let hash = format!("{:x}", sha256.finalize());
    users
        .filter(nickname.eq(nick).and(password.eq(hash)))
        .first(&mut connection)
}
