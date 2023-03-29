//! This file hash database models for user

use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Debug, AsChangeset)]
pub struct User {
    pub id: u64,
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_id: i32,
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
