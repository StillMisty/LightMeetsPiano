pub mod music;

#[tauri::command]
async fn press_key(key: String) {
    music::press_key(&key).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![press_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
