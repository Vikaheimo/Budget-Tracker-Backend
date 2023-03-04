pub mod connect;
pub mod users;

#[derive(PartialEq, Debug)]
pub enum DatabaseError{
    AlreadyExists,
    DoesNotExist,
    ConnectionFailed,
}