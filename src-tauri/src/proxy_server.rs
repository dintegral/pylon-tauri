use actix_web::{error::ErrorBadRequest, web, App, HttpRequest, HttpServer, Result};

use crate::shutdown::Shutdown;

async fn route_handler(req: HttpRequest) -> Result<&'static str> {
    println!("Req: {:?}", req);

    let canister_id = req
        .match_info()
        .get("canister_id")
        .ok_or(ErrorBadRequest("Path parameter canister_id is required"))?;
    let path = req.match_info().get("path").unwrap_or("");

    println!("Canister ID: {}", canister_id);
    println!("Path: {}", path);

    Ok("Hello World!")
}

pub async fn run_proxy_server(mut shutdown: Shutdown) {
    let web_address = "127.0.0.1";
    let web_port = 8080;

    println!(
        "Starting proxy server at http://{}:{}...",
        web_address, web_port
    );

    let server = HttpServer::new(move || {
        App::new()
            .route("/{canister_id}/{path:.*}", web::get().to(route_handler))
            .route("/{canister_id}", web::get().to(route_handler))
    })
    .bind((web_address, web_port))
    .expect("Failed to start proxy server")
    .run();
    let server_handle = server.handle();

    let server_task = tokio::spawn(server);

    shutdown.recv().await;

    println!("Stopping proxy server...");
    server_handle.stop(true).await;
    let _ = server_task.await;
    println!("proxy server stopped!");

    shutdown.send_complete().await;
}
