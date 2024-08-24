use crate::api::routes;
use crate::core::runner;
use queues::*;

pub async fn start_server() {
    let mut job_queue = queue![];
    println!("Starts the web server");
    routes::register_routes(&mut job_queue).await;
    let _ = runner::async_runner().await;
}
