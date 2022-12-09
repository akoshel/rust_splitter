// mod compare_operators;

// use super::compare_operators;
use crate::splitter_config::compare_operators;

pub fn eq_operator(item1: &String, item2: &String) -> bool {
    item1 == item2
}


struct Selector {
    // operator: eq_operator,
    validate_operator: fn(&String, &String) -> bool,
    value: String,
}


impl Selector {
    pub fn new(operator_inp: String, value_type: String, value_inp: String) -> Self {
        let operator = match &operator_inp[..] {
            "eq" => eq_operator,
            // "ge" => compare_operators::ge,
            // "gt" => compare_operators::gt,
            _ => panic!("123"),
        };
        // let val = match &value_type[..] {
        //     "str" => value_inp,
        //     // "int" => value.parse::<i32>().unwrap(),
        //     _ => panic!("321"),
        // };
        Selector {
            validate_operator: operator,
            value: value_inp,
        }
    }
    pub fn validate(&self, value: String) -> bool {
        (self.validate_operator)(&self.value, &value)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector() {
        let operator = String::from("eq");
        let value_type = String::from("str");
        let value = String::from("android");
        let selector = Selector::new(operator, value_type, value);
        assert!(selector.validate("ios".to_string()) == false)
    }
}
