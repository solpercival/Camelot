#[derive(Debug,Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub server_port: u16,
}

impl Config {
    
    pub fn init() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    Config {
            database_url,
            jwt_secret,
            jwt_maxage: jwt_maxage
                .map(|s| s.parse().expect("JWT_MAXAGE must be a valid integer"))
                .unwrap_or(3600), // Default to 1 hour if not set
            server_port: std::env::var("SERVER_PORT")
                .map(|s| s.parse().expect("SERVER_PORT must be a valid integer"))
                .unwrap_or(8080), // Default to 8080 if not set
        }
    }
}