#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("Error while building Tauri application");

    app.run(|_app_handle, _event| {});

    Ok(())
}
