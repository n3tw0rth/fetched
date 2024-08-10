use crate::core::runner;

pub async fn start_server() {
    println!("Starts the web server");
    register_routes().await;
    let _ = runner::async_runner().await;
}

pub async fn register_routes() {}
