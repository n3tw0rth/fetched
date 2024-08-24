use pipeline::api::server;
use pipeline::utils::logging;

#[tokio::main]
async fn main() {
    logging::logger().await;
    server::start_server().await;
}
