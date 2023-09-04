use std::net::SocketAddr;

use color_eyre::Result;

// App configuration
pub struct Config {
    pub host: String,
    pub port: String,
    pub addr: SocketAddr,
}

/// Creates a new Config instance.
pub fn read() -> Result<Config> {
    // Get host and port from environment variables
    let mut host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("2908".to_string());
    if host == "localhost" {
        host = "127.0.0.1".to_string();
    }
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    Ok(Config { host, port, addr })
}
