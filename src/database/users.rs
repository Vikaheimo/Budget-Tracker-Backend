
use crate::{models::user::{NewUser, User}, database::connect};
use diesel::prelude::*;
use crate::database::DatabaseError;

/// Creates a new user on the database and returns the new user's id.
pub fn create_user(new_user: NewUser) -> Result<u64, DatabaseError> {
    use crate::schema::users::dsl::*;

    // Check if user exists with the same username
    match get_user_by_username(&new_user.username) {
        Ok(_) => return Err(DatabaseError::AlreadyExists),
        Err(error) if error == DatabaseError::DoesNotExist => (),
        Err(_) => return Err(DatabaseError::ConnectionFailed),
    }

    // Generate the user
    let mut connection = connect::establish_connection();
     match diesel::insert_into(users).values(&new_user).execute(&mut connection) {
        Ok(_) => (),
        Err(_) => return Err(DatabaseError::ConnectionFailed),
     }

    // Get the user id to handle auth later
    let user = get_user_by_username(&new_user.username)?;
    Ok(user.id)

}

pub fn get_user_by_username(query: &str) -> Result<User, DatabaseError>{
    use crate::schema::users::dsl::*;
    

    let mut connection = connect::establish_connection();
    match users.filter(username.eq(query)).first::<User>(&mut connection) {
        Ok(user) => Ok(user),
        Err(error) if error == diesel::result::Error::NotFound => Err(DatabaseError::DoesNotExist),
        Err(_) => Err(DatabaseError::ConnectionFailed),
    }
}
