use async_trait::async_trait;
use config::{AsyncSource, Config, ConfigError, Format, Map};
use std::fmt::Debug;

fn main() {
    let _ = dotenvy::dotenv();

    let testvar = std::env::var("TESTVAR").unwrap_or_else(|_| "default".to_string());

    let settings_reader = Config::builder()
        .add_source(config::File::with_name("settings").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    println!("{testvar}");
}

#[derive(Debug)]
struct HttpSource<F: Format> {
    uri: String,
    format: F,
}

#[async_trait]
impl<F: Format + Send + Sync + Debug> AsyncSource for HttpSource<F> {
    async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
        reqwest::get(&self.uri)
            .await
            .map_err(|e| ConfigError::Foreign(Box::new(e)))?
            .text()
            .await
            .map_err(|e| ConfigError::Foreign(Box::new(e)))
            .and_then(|text| {
                self.format
                    .parse(Some(&self.uri), &text)
                    .map_err(ConfigError::Foreign)
            })
    }
}
