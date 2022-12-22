#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//use tauri::Manager;

fn main() {
    tauri::Builder::default()
        /*.setup(|app| {
            app.asset_protocol_scope().allow_directory(std::env::current_dir()?, true)?;

            Ok(())
        })*/
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}