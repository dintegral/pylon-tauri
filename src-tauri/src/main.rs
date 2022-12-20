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

const NNS_CANISTER_ID: &'static str = "qoctq-giaaa-aaaaa-aaaea-cai";

fn main() {
    let (shutdown_tx, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("Error while building Tauri application");

    {
        let shutdown = Shutdown::new(shutdown_tx.subscribe(), shutdown_complete_tx.clone());

        tauri::async_runtime::spawn(async {
            run_proxy_server(shutdown).await;
        });
    }

    let url = format!("http://localhost:8080/{}", NNS_CANISTER_ID);
    tauri::WindowBuilder::new(
        &app,
        "nns",
        tauri::WindowUrl::External(url.parse().unwrap()),
    )
    .build()
    .expect("Error while building Tauri window");

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
