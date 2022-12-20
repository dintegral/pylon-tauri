use actix_web::{
    error::{ErrorBadGateway, ErrorBadRequest},
    http::header::ContentType,
    web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use ic_agent::export::Principal;
use ic_utils::interfaces::http_request::HeaderField;

use crate::{agent::create_agent, canister::canister_http_request, shutdown::Shutdown};

const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";

async fn route_handler(req: HttpRequest) -> Result<HttpResponse> {
    let canister_id = req
        .match_info()
        .get("canister_id")
        .ok_or(ErrorBadRequest("Path parameter canister_id is required"))?;
    let canister_id = Principal::from_text(canister_id)
        .map_err(|_| ErrorBadRequest("Path parameter canister_id must be a valid Principal"))?;
    let path = req.match_info().get("path").unwrap_or("/");

    let agent = create_agent(DEFAULT_IC_GATEWAY);

    let headers = req
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            Some(HeaderField(
                name.as_str().into(),
                value.to_str().ok()?.into(),
            ))
        })
        .collect();
    let res = canister_http_request(&agent, &canister_id, &path, headers)
        .await
        .expect("Failed to fetch file");

    let body = String::from_utf8(res.body)
        .map_err(|_| ErrorBadGateway("Invalid utf-8 string returned from replica"))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
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
