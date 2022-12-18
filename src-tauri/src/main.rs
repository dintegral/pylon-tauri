#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

mod file_server;
use file_server::FileServer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("Error while building Tauri application");

    let data_dir = app.path_resolver().app_data_dir().unwrap();
    let dapps_dir = data_dir.join("dapps");

    let file_server = Arc::new(FileServer::new(dapps_dir.clone()));
    let app_handle = Arc::new(app.handle());

    let ctrlc_app_handle = app_handle.clone();
    let ctrlc_file_server = file_server.clone();
    ctrlc::set_handler(move || {
        ctrlc_file_server.stop();
        ctrlc_app_handle.exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let app_file_server = file_server.clone();
    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::Exit {} => {
            app_file_server.stop();
        }
        _ => {}
    });

    Ok(())
}
