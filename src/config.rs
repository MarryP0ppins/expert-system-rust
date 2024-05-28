use tower_cookies::Key;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub frontend_origin: String,

    pub cookie_key: Key,
    pub crypto_key: String,
    pub nonce_key: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let frontend_origin =
            std::env::var("FRONTEND_ORIGIN").expect("FRONTEND_ORIGIN must be set");

        let cookie_key = std::env::var("COOKIE_KEY").expect("COOKIE_KEY must be set");
        let crypto_key = std::env::var("CRYPTO_KEY").expect("CRYPTO_KEY must be set");
        let nonce_key = std::env::var("NONCE_KEY").expect("NONCE_KEY must be set");

        let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = std::env::var("SMTP_PORT").expect("SMTP_PORT must be set");
        let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = std::env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_from = std::env::var("SMTP_FROM").expect("SMTP_FROM must be set");

        Config {
            database_url,
            frontend_origin,
            cookie_key: Key::from(cookie_key.as_bytes()),
            crypto_key,
            nonce_key,
            smtp_host,
            smtp_pass,
            smtp_user,
            smtp_port: smtp_port.parse::<u16>().unwrap(),
            smtp_from,
        }
    }
}
