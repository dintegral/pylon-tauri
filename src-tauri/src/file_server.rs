use std::path::PathBuf;

use actix_files::Files;
use actix_web::{App, HttpServer};

use crate::shutdown::Shutdown;

pub async fn run_file_server(dir: PathBuf, mut shutdown: Shutdown) {
    let web_address = "127.0.0.1";
    let web_port = 8080;

    println!(
        "Starting file server at http://{}:{}...",
        web_address, web_port
    );

    let server =
        HttpServer::new(move || App::new().service(Files::new("/", dir.clone()).prefer_utf8(true)))
            .bind((web_address, web_port))
            .expect("Failed to start file server")
            .run();
    let server_handle = server.handle();

    let server_task = tokio::spawn(server);

    shutdown.recv().await;

    println!("Stopping file server...");
    server_handle.stop(true).await;
    let _ = server_task.await;
    println!("File server stopped!");

    shutdown.send_complete().await;
}
