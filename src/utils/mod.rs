use std::env::{self, VarError};

pub struct ApiConfig {
    pinecone_api_key: String,
    openai_api_key: String,
}

impl ApiConfig {
    pub const fn new(pinecone_api_key: String, openai_api_key: String) -> ApiConfig {
        ApiConfig {
            pinecone_api_key,
            openai_api_key,
        }
    }
}

pub fn get_api_configs() -> Result<ApiConfig, VarError> {
    let pinecone_api_key = env::var("PINECONE_API_KEY")?;

    let openai_api_key = env::var("OPENAI_API_KEY")?;

    Ok(ApiConfig {
        pinecone_api_key,
        openai_api_key,
    })
}
