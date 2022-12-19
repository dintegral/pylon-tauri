#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod file_server;
mod shutdown;

use tokio::sync::{broadcast, mpsc};

use file_server::run_file_server;
use shutdown::Shutdown;

fn main() {
    let (shutdown_tx, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("Error while building Tauri application");

    let data_dir = app.path_resolver().app_data_dir().unwrap();
    let dapps_dir = data_dir.join("dapps");

    {
        let shutdown = Shutdown::new(shutdown_tx.subscribe(), shutdown_complete_tx.clone());

        tauri::async_runtime::spawn(async {
            run_file_server(dapps_dir, shutdown).await;
        });
    }

    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::Exit {} => {
            println!("Shutting down...");
            let _ = shutdown_tx.send(());
            let _ = shutdown_complete_rx.blocking_recv();
            println!("Shutdown complete!");
        }
        _ => {}
    });
}
