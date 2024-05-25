use std::env::var;

use dotenv::dotenv;
use env_logger::{init_from_env, Env};

pub mod server;

pub fn load_env() {
    log::info!("Loading .env file");
    dotenv().ok();
}

pub fn init_logger() {
    init_from_env(Env::new().default_filter_or("info"));
}

pub fn load_port() -> String {
    var("PORT").unwrap_or_else(|_| "8080".to_string())
}

pub fn load_bind() -> String {
    let ip = var("IPV4").unwrap_or_else(|_| "0.0.0.0".to_string());
    let bind = format!("{}:{}", ip, load_port());
    log::info!("Binding to {}:{}", ip, load_port());
    bind
}

pub fn load_bind_ipv6() -> String {
    let ip = var("IPV6").unwrap_or_else(|_| "::1".to_string());
    let bind = format!("[{}]:{}", ip, load_port());
    log::info!("Binding to [{}]:{}", ip, load_port());
    bind
}
