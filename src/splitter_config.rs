// use std::cmp::{Eq, PartialEq}
use config::{Config, ConfigError, File};
use serde::Deserialize;

mod compare_operators;
mod selector;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Group {
    name: String,
    ratio: f32,
    meta_data: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SelectorConfig {
    name: String,
    // Literal["str", "bool", "float", "int", "set[str]", "set[int]"]
    value_type: String,
    // Literal["lt", "gt", "le", "ge", "eq", "ne", "contains", "not_contains"]
    operator: String,
    value: String,
}

impl SelectorConfig {
    pub fn validate(&self, value: &String) -> bool {
        let compare_fn = match &self.operator[..] {
            "eq" => compare_operators::eq,
            "ge" => compare_operators::ge,
            "gt" => compare_operators::gt,
            _ => panic!("Incorrect compare operator!")
        };
        compare_fn(&self.value, &value)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ExperimentConfig {
    group: Group,
    selector: SelectorConfig,
}

impl ExperimentConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(File::with_name("config/splitter.toml"))
            .build()?;
        conf.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_settings() {
        let app_settings = ExperimentConfig::new();
        println!("{:?}", app_settings);
        assert!(app_settings.is_ok())
    }

    #[test]
    fn test_selector() {
        let selector = SelectorConfig {
            name: String::from("123"),
            value_type: String::from("str"),
            operator: String::from("eq"),
            value: String::from("android"),
        };
        let check1 = selector.validate(&String::from("android"));
        let check2 = selector.validate(&String::from("ios"));
        assert!(check1);
        assert!(check2 == false);
    }
}
