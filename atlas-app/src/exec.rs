use tauri::command;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TableData {
    pub date: String,
    pub name: String,
    pub address: String,
}

#[command]
pub fn update_data(row: TableData) -> Vec<TableData> {
    println!("{}-{}", row.name, row.date);

    let mut data = vec![row.clone()];
    data.push(row.clone());
    data.push(row.clone());
    data.push(row.clone());
    data
}
