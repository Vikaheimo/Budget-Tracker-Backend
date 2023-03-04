use std::env;

use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Debug, AsChangeset)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
