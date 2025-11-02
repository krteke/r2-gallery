use secrecy::SecretString;

use crate::config::Config;

pub struct AppState {
    pub username: SecretString,
    pub password: SecretString,
    pub jwt_secret: SecretString,
}

impl AppState {
    pub fn new(config: Config) -> Self {

        AppState {
            username: config.username,
            password: config.password,
            jwt_secret: config.jwt_secret,
        }
    }
}