use secrecy::SecretString;

#[derive(Debug, Clone)]
pub struct Config {
    pub username: SecretString,
    pub password: SecretString,
    pub jwt_secret: SecretString,
}

impl Config {
    pub fn load_config() -> Self {
        let username = std::env::var("USERNAME").expect("USER_NAME environment variable not set").into();
        let password = std::env::var("PASSWORD").expect("PASSWORD environment variable not set").into();
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable not set").into();

        Config {
            username,
            password,
            jwt_secret,
        }
    }
}
