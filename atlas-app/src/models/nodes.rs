use chrono::NaiveDateTime;
use diesel::prelude::*;
use tauri::State;
use crate::Pool;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::nodes)]
pub struct Node {
    pub id: Option<i32>,
    pub server_name: String,
    pub ip_address: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
}

#[tauri::command]
pub async fn get_nodes(pool: State<'_, Pool>) -> Result<Vec<Node>, String> {
    use crate::schema::nodes::dsl::*;

    let mut conn = pool.get().map_err(|e| e.to_string())?;

    nodes.load::<Node>(&mut conn).map_err(|e| e.to_string())
}
