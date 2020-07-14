use crate::test_fns;
use non_none_fields::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::string::String;

trait Digr_Test {
    fn rules_ops(&self, elem: &String);
    fn assertions_ops(
        &self,
        assertion_value: &str,
        assertion_index: usize,
        elem: &String,
    ) -> Option<bool>;
    fn find_global_var(&self, variable_name: &str, index: usize) -> Option<String>;
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
struct TestValue {
    pub name: String,
    pub r#if: Option<Vec<String>>,
    pub r#let: Option<HashMap<String, String>>,
    pub links: Option<String>,
    pub ifEquals: Option<Vec<String>>,
    pub ifNotEquals: Option<Vec<String>>,
    pub ifGreaterThan: Option<Vec<String>>,
    pub ifLessThan: Option<Vec<String>>,
    pub ifGreaterThanOrEquals: Option<Vec<String>>,
    pub ifLessThanOrEquals: Option<Vec<String>>,
    pub ifNull: Option<String>,
    pub ifNotNull: Option<String>,
    pub ifIncludes: Option<String>,

    pub assert: Option<Vec<String>>,
    pub assertEquals: Option<Vec<String>>,
    pub assertGreaterThan: Option<Vec<String>>,
    pub assertLessThan: Option<Vec<String>>,
    pub assertGreaterThanOrEquals: Option<Vec<String>>,
    pub assertLessThanOrEquals: Option<Vec<String>>,
    pub assertNull: Option<String>,
 
    pub assertNotGreaterThan: Option<Vec<String>>,
    pub assertNotEquals: Option<Vec<String>>,
    pub assertNotNull: Option<String>,
} 

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
struct ValidationValue {
    name: String,
    case: String,
    assert: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
pub struct RuleSpec {
    pub name: String,
    pub meta: Option<HashMap<String, String>>,
    pub on: Vec<String>,
    #[serde(default = "default_hidden")]
    pub includeHidden: bool,
    pub tests: Vec<TestValue>,
    pub validations: Vec<ValidationValue>,
}

impl Digr_Test for RuleSpec {
    fn rules_ops(&self, elem: &String) {
        for iter in self.tests.iter().zip(self.validations.iter()) {
            let (test_case, validation) = iter;
            let fields = test_case.non_none_fields();
            let mut test_results: Vec<HashMap<String, String>> = vec![];
            for (index, &field) in fields.iter().enumerate() {
                println!("{}", field);

                match field {
                    "if" => {
                        let rule_value = test_case.r#if.as_ref().unwrap();
                        let ref current_value = &rule_value[0];
                        let ref expected_value = &rule_value[1];
                        if Some(true) =
                            test_fns::test_equals(&current_value, &expected_value, &elem)
                        {
                            let ref assertion_value = fields[2];

                            match self.assertions__ops(assertion_value, index, &elem) {
                                Some(value) => {
                                    if value {
                                        let mut result = HashMap::new();
                                        if test_case.name == validation.name {};

                                    // / Need to speak to Darius
                                    // / Not sure if tests and validations
                                    // / are in same order
                                    // assert_eq!(assertion_name, validation_name, "")
                                    // result.insert(tes, v: V)
                                    // test_results.push()
                                    } else {
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    fn assertions_ops(
        &self,
        assertion_value: &str,
        assertion_index: usize,
        elem: &String,
    ) -> Option<bool> {
        match assertion_value {
            "assert" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assert.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                test_fns::test_equals(&current_value, &expected_value, &elem)
            }
            "assertEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                test_fns::test_equals(&current_value, &expected_value, &elem)
            }
            "assertNotEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertNotEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                match test_fns::test_equals(&current_value, &expected_value, &elem) {
                    Some(true) => Some(false),
                    Some(false) => Some(true),
                    None => None,
                }
            }
            "assertGreaterThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertGreaterThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                test_fns::test_greater_than(&current_value, &expected_value, &elem)
            }
            "assertLessThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertLessThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];

                test_fns::test_less_than(&current_value, &expected_value, &elem)
            }
            "assertGreaterThanOrEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertGreaterThanOrEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                let eq = test_fns::test_equals(&current_value, &expected_value, &elem).unwrap();
                let gt = test_fns::test_greater_than(current_value, expected_value, &elem).unwrap();
                if eq || gt {
                    return Some(true);
                } else {
                    return Some(false);
                }
            }
            "assertNotGreaterThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertNotGreaterThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                let gt = test_fns::test_greater_than(current_value, expected_value, &elem).unwrap();
                if gt {
                    return Some(false);
                } else {
                    return Some(true);
                }
            }
            "assertLessThanOrEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertGreaterThanOrEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                let eq = test_fns::test_equals(&current_value, &expected_value, &elem).unwrap();
                let lt = test_fns::test_less_than(current_value, expected_value, &elem).unwrap();
                if eq || lt {
                    return Some(true);
                } else {
                    return Some(false);
                }
            }
            "assertNull" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assert.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                test_fns::test_equals(&current_value, &expected_value, &elem)
            }
            "assertNotNull" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assert.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                match test_fns::test_equals(&current_value, &expected_value, &elem) {
                    Some(true) => Some(false),
                    Some(false ) => Some(true),
                }
            }
            _ => None,
        }
    }

    fn find_global_var(&self, variable_name: &str, index: usize) -> Option<String>{
        let ref test = &self.tests[index];
        let ref test_let = &test.r#let.as_ref().unwrap();
        if !test_let.contains_key(variable_name) {
            return None;
        }
        match variable_name {}
    }
}
fn default_hidden() -> bool {
    false
}
