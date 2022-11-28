#[macro_use]
pub mod splitter_config;


fn main() {
    let app_settings = splitter_config::ExperimentConfig::new().unwrap();
    println!("Hello, world!");
}
