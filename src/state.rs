#[cfg(feature = "server")]
pub mod app {
    use aws_config::{BehaviorVersion, Region};
    use aws_sdk_s3::Client;
    use secrecy::SecretString;

    use crate::config::Config;

    pub struct AppState {
        pub username: SecretString,
        pub password: SecretString,
        pub jwt_secret: SecretString,
        pub r2_client: Client,
        pub r2_bucket: String,
    }

    impl AppState {
        pub async fn new(config: Config) -> Self {
            let r2_url = format!("https://{}.r2.cloudflarestorage.com", config.r2_account_id);
            let r2_region = Region::new(config.r2_region);

            let sdk_config = aws_config::defaults(BehaviorVersion::latest())
                .region(r2_region)
                .endpoint_url(r2_url)
                .load()
                .await;

            let r2_client = Client::new(&sdk_config);

            tracing::info!("R2 client initialized");

            AppState {
                username: config.username,
                password: config.password,
                jwt_secret: config.jwt_secret,
                r2_client,
                r2_bucket: config.r2_bucket_name,
            }
        }
    }
}
