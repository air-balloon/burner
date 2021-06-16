use chrono::NaiveDateTime;

use crate::models::enums::AccountStatus;
use crate::schema::users;

#[derive(Queryable, Identifiable)]
#[primary_key(uuid)]
#[column_name(uuid)]
pub struct User {
    pub uuid: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub account_status: AccountStatus,
    pub timezone: Option<String>,
    pub first_log_in_at: Option<NaiveDateTime>,
    pub last_log_in_at: Option<NaiveDateTime>,
    pub language: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub uuid: &'a Vec<u8>,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: Option<&'a NaiveDateTime>,
    pub username: &'a String,
    pub first_name: &'a String,
    pub last_name: &'a String,
    pub account_status: &'a AccountStatus,
    pub timezone: Option<&'a String>,
    pub first_log_in_at: Option<&'a NaiveDateTime>,
    pub last_log_in_at: Option<&'a NaiveDateTime>,
    pub language: Option<&'a String>,
}
