#[macro_use]
extern crate serde;
extern crate serde_yaml;

use std::string::String;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use smol;
use surf;
use anyhow::{Error};
use scraper::{Html, Selector};
use non_none_fields::*;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
struct TestValue {
    name: String,
    r#if: Option<Vec<String>>,
    r#let: Option<HashMap<String, String>>,
    links: Option<String>,
    ifEquals: Option<Vec<String>>,
    ifNotEquals: Option<Vec<String>>,
    ifGreaterThan: Option<Vec<String>>,
    ifLessThan: Option<Vec<String>>,
    ifGreaterThanOrEquals: Option<Vec<String>>,
    ifLessThanOrEquals: Option<Vec<String>>,
    ifNull: Option<String>,
    ifNotNull: Option<String>,
    ifIncludes: Option<String>,

    assert: Option<Vec<String>>,
    assertEquals: Option<Vec<String>>,
    assertGreaterThan: Option<Vec<String>>,
    assertLessThan: Option<Vec<String>>,
    assertGreaterThanOrEquals: Option<Vec<String>>,
    assertLessThanOrEquals: Option<Vec<String>>,
    assertNull: Option<String>,

    assertNotGreaterThan: Option<Vec<String>>,
    assertNotEquals: Option<Vec<String>>,
    assertNotNull: Option<String>,
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
struct RuleSpec {
    name: String,
    meta: Option<HashMap<String, String>>,
    on: Vec<String>,
    #[serde(default = "default_hidden")]
    includeHidden: bool,
    tests: Vec<TestValue>,
    validations: Vec<ValidationValue>,
}

impl RuleSpec {
    fn test_rules_ops(self, elem: &String) {
        for iter in self.tests.iter().zip(self.validations.iter()) {
            let (test_case, validation) = iter;
            let fields = test_case.non_none_fields();
            for &field in fields.iter() {
                println!("{}", field);
            
                match field {
                    "if" => {                      
                        let rule_value = test_case.r#if.as_ref().unwrap();
                        let ref current_value = &rule_value[0];
                        let ref expected_value = &rule_value[1];
                        if let Some(()) = test_fns::test_equals(&current_value, &expected_value, &elem) {
                            let ref assertion_value = fields[2];
                        }   
                    },
                    _ => continue,
                }
            }
        }
    
    }

    fn test_assertions__ops(self, value: &str, assertion_index: usize, elem: &String) -> Option<()> {
        match value {
            "assert" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assert.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];

                test_fns::test_equals(&current_value, &expected_value, &elem){
            },
            "assertEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];

                test_fns::test_equals(&current_value, &expected_value, &elem);
            },
            "assertNotEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertNotEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];

                match test_fns::test_equals(&current_value, &expected_value, &elem) {
                    Some(()) => None,
                    None => Some(())
                }
            },
            "assertGreaterThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertGreaterThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];

                test_fns::test_greater_than(&current_value, &expected_value, &elem);
            },
            "assertLessThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertLessThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                match test_fns::test_equals(&current_value, &expected_value, &elem) {
                    Some(()) => None,
                    None => {
                        test_fns::test_greater_than(&current_value, &expected_value, &elem) {
                            Some(()) => None,
                            None => Some(())
                        }
                    }
                }
                
            },
            "assertGreaterThanOrEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertGreaterThanOrEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                match test_fns::test_equals(&current_value, &expected_value, &elem) {
                    Some => {
                        test_fns::test_greater_than(&current_value, &expected_value, &elem) {
                            Some(()) => Some(()),
                            None => Some()
                        }
                    },
                    None => {
                        test_fns::test_greater_than(&current_value, &expected_value, &elem) {
                            Some(()) => Some(()),
                            None => None
                        }
                    },
                }
                
            },
            "assertNotGreaterThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertNotGreaterThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];

                test_fns::test_greater_than(&current_value, &expected_value, &elem);
            },
            _ => None
        }

    }
}
fn default_hidden() -> bool {
    false
}