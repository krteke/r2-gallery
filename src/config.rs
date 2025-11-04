use secrecy::SecretString;

#[derive(Debug, Clone)]
pub struct Config {
    pub username: SecretString,
    pub password: SecretString,
    pub jwt_secret: SecretString,
    pub r2_account_id: String,
    pub r2_region: String,
    pub r2_bucket_name: String,
}

impl Config {
    pub fn load_config() -> Self {
        let username = std::env::var("USERNAME")
            .expect("USER_NAME environment variable not set")
            .into();
        let password = std::env::var("PASSWORD")
            .expect("PASSWORD environment variable not set")
            .into();
        let jwt_secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET environment variable not set")
            .into();
        let r2_account_id =
            std::env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID environment variable not set");
        let r2_region = std::env::var("R2_REGION").expect("R2_REGION environment variable not set");
        let r2_bucket_name =
            std::env::var("R2_BUCKET_NAME").expect("R2_BUCKET_NAME environment variable not set");

        Config {
            username,
            password,
            jwt_secret,
            r2_account_id,
            r2_region,
            r2_bucket_name,
        }
    }
}
