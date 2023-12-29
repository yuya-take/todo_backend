use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::users;


#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}