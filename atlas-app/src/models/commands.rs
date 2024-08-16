use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::commands)]
pub struct Command {
    pub id: Option<i32>,
    pub name: String,
    pub command_text: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}
