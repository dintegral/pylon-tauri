use std::{path::PathBuf, sync::mpsc, thread};

use actix_files::Files;
use actix_web::{dev::ServerHandle, rt, App, HttpServer};

pub struct FileServer {
    server_handle: ServerHandle,
}

impl FileServer {
    pub fn new(dir: PathBuf) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let web_address = "127.0.0.1";
            let web_port = 8080;

            println!(
                "Starting file server at http://{}:{}...",
                web_address, web_port
            );

            let server = HttpServer::new(move || {
                App::new().service(Files::new("/", dir.clone()).prefer_utf8(true))
            })
            .bind((web_address, web_port))
            .expect("Failed to start file server")
            .run();

            tx.send(server.handle())
                .expect("Failed to return file server handle from thread");

            rt::System::new().block_on(server)
        });

        let server_handle = rx
            .recv()
            .expect("Failed to get handle from file server thread");

        FileServer { server_handle }
    }

    pub fn stop(&self) {
        println!("Stopping file server...");
        rt::System::new().block_on(self.server_handle.stop(true));
    }
}
