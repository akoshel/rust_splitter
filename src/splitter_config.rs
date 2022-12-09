// use std::cmp::{Eq, PartialEq}
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::hash::Hash;

mod compare_operators;
mod selector;

struct Query {
    request: ExperimentInput,
    address: String,
}

#[derive(Debug, Hash)]
pub struct ExperimentInput {
    user_id: String,
    value: String,
}

impl ExperimentInput {
    pub fn new(user_id_inp: &String, value_inp: &String) -> Self {
        ExperimentInput {
            user_id: user_id_inp.to_string(),
            value: value_inp.to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Group {
    name: String,
    ratio: f32,
    meta_data: String,
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
    pub fn validate(&self, value: &ExperimentInput) -> bool {
        let compare_fn = match &self.operator[..] {
            "eq" => compare_operators::eq,
            "ge" => compare_operators::ge,
            "gt" => compare_operators::gt,
            _ => panic!("Incorrect compare operator!"),
        };
        let is_selected = compare_fn(&self.value, &value.value);
        match is_selected {
            true => true,
            false => false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ExperimentConfig {
    group: Group,
    pub selector: SelectorConfig,
}

impl ExperimentConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            .add_source(File::with_name("config/splitter.toml"))
            .build()?;
        conf.try_deserialize()
    }

    pub fn validate(&self, value: ExperimentInput) -> Query {
        let server_address = match self.selector.validate(&value) {
            false => "http://localhost:8005/".to_string(),
            true => self.group.meta_data.clone(),
        };
        Query {
            request: value,
            address: server_address,
        }
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
        let check_val1 = ExperimentInput::new(&String::from("android"), &String::from("android"));
        let check_val2 = ExperimentInput::new(&String::from("android"), &String::from("ios"));
        let check1 = selector.validate(&check_val1);
        let check2 = selector.validate(&check_val2);
        assert!(check1);
        assert!(check2 == false);
    }
}
