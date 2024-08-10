use crate::core::runner;

pub fn start_server() {
    println!("Starts the web server");
    register_routes();
    runner::async_runner();
}

pub fn register_routes() {}
