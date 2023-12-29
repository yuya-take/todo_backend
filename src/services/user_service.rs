use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use crate::db::connection::establish_connection;
use crate::schema::users;

pub fn create_user(new_user: NewUser) -> QueryResult<User> {
    let mut conn = establish_connection().unwrap();
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
}

pub fn get_user(user_id: i32) -> QueryResult<User> {
    let mut conn = establish_connection().unwrap();
    users::table.find(user_id).get_result::<User>(&mut conn)
}

pub fn get_users() -> QueryResult<Vec<User>> {
    let mut conn = establish_connection().unwrap();
    users::table.load::<User>(&mut conn)
}

pub fn delete_user(user_id: i32) -> QueryResult<usize> {
    let mut conn = establish_connection().unwrap();
    diesel::delete(users::table.find(user_id)).execute(&mut conn)
}