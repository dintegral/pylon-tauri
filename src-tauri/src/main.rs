#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod agent;
mod canister;
mod proxy_server;
mod shutdown;

use tokio::sync::{broadcast, mpsc};

use proxy_server::run_proxy_server;
use shutdown::Shutdown;

#[tauri::command]
async fn open_dapp(handle: tauri::AppHandle, canister_id: String) -> Result<(), tauri::Error> {
    return match open_dapp_window(&handle, &canister_id) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}

fn open_dapp_window(
    handle: &tauri::AppHandle,
    canister_id: &str,
) -> Result<tauri::Window, tauri::Error> {
    let url = format!("http://localhost:8080/{}", canister_id);

    tauri::WindowBuilder::new(
        handle,
        canister_id,
        tauri::WindowUrl::External(url.parse().unwrap()),
    )
    .build()
}

fn main() {
    let (shutdown_tx, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_dapp])
        .build(tauri::generate_context!())
        .expect("Error while building Tauri application");

    {
        let shutdown = Shutdown::new(shutdown_tx.subscribe(), shutdown_complete_tx.clone());

        tauri::async_runtime::spawn(async {
            run_proxy_server(shutdown).await;
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
