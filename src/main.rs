mod api;
use crate::api::{gpt, pinecone};
use rAutoGPT::utils;

fn main() {
    let configs = match utils::get_api_configs() {
        Ok(config) => config,
        Err(_) => {
            panic!("Unable to load configurations!")
        }
    };
    println!("Starting rAutopGPT...")
}
