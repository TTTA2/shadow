// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn db_connect() -> Result<sqlite::Connection, sqlite::Error> {
    let connection = sqlite::open("./test.sqlite");
    return connection;
}

fn db_init() -> Result<i32, String> {

    let connection = db_connect().unwrap();

    let query = "
    CREATE TABLE templates (name TEXT, content TEXT);
    INSERT INTO templates VALUES ('test1', 'aaaaaaa');
    INSERT INTO templates VALUES ('test2', 'bbbbbb');";

    let err = connection.execute(query);

    println!("{}", err.is_err().to_string());

    Ok(0)
}

fn db_all(connection: sqlite::Connection) {
    
    let query = "SELECT * FROM templates";

    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .unwrap();
}


fn main() {

    db_init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
