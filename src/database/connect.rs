use diesel::{mysql::MysqlConnection, Connection};
use std::env;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").unwrap();

    MysqlConnection::establish(&database_url).unwrap()
}