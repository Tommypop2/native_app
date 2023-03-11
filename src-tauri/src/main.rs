#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let result = window
                .eval("window.location.replace('https://language-whim4-0-frontend.vercel.app')");
            if result.is_ok() {
                Ok(())
            } else {
                panic!("Eval Failed")
            }
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
