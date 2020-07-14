#[macro_use]
extern crate serde;
extern crate serde_yaml;

use anyhow::Error;
use non_none_fields::*;
use scraper::{Html, Selector};
use smol;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::string::String;
use structopt::StructOpt;
use surf;

use crate:: test_fns;

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
pub struct RuleSpec {
    pub name: String,
    pub meta: Option<HashMap<String, String>>,
    pub on: Vec<String>,
        #[serde(default = "default_hidden")]
    pub includeHidden: bool,
    pub tests: Vec<TestValue>,
    pub validations: Vec<ValidationValue>,
}

impl RuleSpec {
    fn test_rules_ops(self, elem: &String) {
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
                        if Some(true) = test_fns::test_equals(&current_value, &expected_value, &elem)
                        {
                            let ref assertion_value = fields[2];

                            match self.test_assertions__ops(assertion_value, index, &elem) {
                                Some(value) => {
                                    if value {
                                        let mut result = HashMap::new();
                                        if test_case.name == validation.name {

                                        };

                                        /// Need to speak to Darius 
                                        /// Not sure if tests and validations 
                                        /// are in same order
                                        /// 
                                        /// 
                                        /// 

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

    fn test_assertions__ops(
        self,
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
                test_fns::test_equals(&current_value, &expected_value, &elem);
            }
            "assertNotEquals" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertNotEquals.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                match test_fns::test_equals(&current_value, &expected_value, &elem) {
                    Some(true) => Some(false),
                    Some(false) => Some(true),
                    None => {}
                }
            }
            "assertGreaterThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertGreaterThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                test_fns::test_greater_than(&current_value, &expected_value, &elem);
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
                    Some(true);
                } else {
                    Some(false);
                }
            }
            "assertNotGreaterThan" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assertNotGreaterThan.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                let gt = test_fns::test_greater_than(current_value, expected_value, &elem).unwrap();
                if gt {
                    Some(false);
                } else {
                    Some(true);
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
                    Some(true);
                } else {
                    Some(false);
                }
            }
            "assertNull" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assert.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                test_fns::test_equals(&current_value, &expected_value, &elem);
            }
            "assertNotNull" => {
                let ref test_case = &self.tests[assertion_index];
                let ref assertion_value = test_case.assert.as_ref().unwrap();
                let ref current_value = &assertion_value[0];
                let ref expected_value = &assertion_value[1];
                match test_fns::test_equals(&current_value, &expected_value, &elem) {
                    Some(true) => Some(false),
                    None => Some(()),
                }
            }
            _ => None,
        }
    }

    fn spec_globals(self, variable_name: &str, index: usize) {
        let ref test = self.tests[index].as_ref().unwrap();
        if test.co .contains_key("Les Misérables") {
            println!("We've got {} reviews, but Les Misérables ain't one.",
                     book_reviews.len());
        }
        match variable_name {

        }
    }
}
fn default_hidden() -> bool {
    false
}
