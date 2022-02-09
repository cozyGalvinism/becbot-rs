use super::schema::{cans, quotes};
use chrono::NaiveDateTime;
use diesel::Insertable;

#[derive(Queryable)]
pub struct Quote {
    pub quote_id: i32,
    pub message: String,
    pub quote_author_id: i64,
    pub quote_author: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "quotes"]
pub struct NewQuote<'a> {
    pub message: &'a str,
    pub quote_author_id: i64,
    pub quote_author: &'a str,
    pub date: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Can {
    pub can_id: i32,
    pub user_id: i64,
    pub user: String,
    pub date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "cans"]
pub struct NewCan<'a> {
    pub user_id: i64,
    pub user: &'a str,
    pub date: NaiveDateTime,
}