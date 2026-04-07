use std::{env, net::SocketAddr};

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(3000);

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Self {
            host,
            port,
            database_url,
            jwt_secret,
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("invalid HOST or PORT")
    }
}
