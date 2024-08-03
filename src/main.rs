use pipeline::api::server;
use pipeline::utils::logging;

fn main() {
    logging::logger();
    server::start_server();
}
