use log::error;
use ob::Connection;
use sqlite::SqliteManager;
use tauri::Manager;

mod ob;
mod sqlite;

#[derive(Default)]
struct AppState {
    sqlite_manager: SqliteManager,
}

fn init_database(sqlite_manager: &SqliteManager) -> Result<(), Box<dyn std::error::Error>> {
    sqlite_manager.init_table::<Connection>()?;
    Ok(())
}

#[tauri::command]
fn create_connection(
    app_handler: tauri::AppHandle,
    uri: String,
    name: String,
    color: String,
) -> Result<i64, String> {
    let state = app_handler.state::<AppState>();

    let connection = Connection {
        id: 0,
        uri_connection: uri,
        name,
        color,
    };

    state
        .sqlite_manager
        .insert(&connection)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_connection(app_handler: tauri::AppHandle, id: i64) -> Result<Option<Connection>, String> {
    let state = app_handler.state::<AppState>();

    state
        .sqlite_manager
        .get_by_id(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_connection(app_handler: tauri::AppHandle, entity: Connection) -> Result<(), String> {
    let state = app_handler.state::<AppState>();

    state
        .sqlite_manager
        .update(&entity)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_connection(app_handler: tauri::AppHandle, id: i64) -> Result<(), String> {
    let state = app_handler.state::<AppState>();

    state
        .sqlite_manager
        .delete::<Connection>(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_connection(app_handler: tauri::AppHandle) -> Result<Vec<Connection>, String> {
    let state = app_handler.state::<AppState>();

    state.sqlite_manager.list().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sqlite_manager = SqliteManager::new().unwrap();

    if let Err(e) = init_database(&sqlite_manager) {
        error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    tauri::Builder::default()
        .manage(AppState { sqlite_manager })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            create_connection,
            get_connection,
            update_connection,
            delete_connection,
            list_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
