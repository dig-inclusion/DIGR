use crate::test_fns;
use non_none_fields::*;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::string::String;

const NULL: &str = "null";

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, NonNoneFields)]
pub struct TestValue {
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
#[derive(Debug, Deserialize)]
pub struct ValidationValue {
    pub name: String,
    pub case: String,
    pub assert: String,
}

#[derive(Debug)]
pub struct OpsResult {
    name: String,
    test_op: String,
    assertion_op: String,
}

impl fmt::Display for OpsResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}\n {}\n {})",
            self.name, self.test_op, self.assertion_op
        )
    }
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
    pub fn rules_ops(&self, selector: &Selector, fragment: &Html) -> Vec<OpsResult> {
        let ts_len = self.tests.len();
        let mut test_results: Vec<OpsResult> = Vec::with_capacity(ts_len);
        for mut test_c in &mut self.tests.iter() {
            let test_case = &mut test_c;
            let fields = test_case.non_none_fields();
            for &field in fields.iter() {
                println!("{}", field);
                let passed = "passed";
                let failed = "failed";
                let nothing = "_";

                match field {
                    "if" => {
                        let rule_value = test_case.r#if.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let test_op_res = &var_value == expected_value;
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    "ifEquals" => {
                        let rule_value = test_case.ifEquals.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let test_op_res = &var_value == expected_value;
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    "ifNotEquals" => {
                        let rule_value = test_case.ifNotEquals.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let test_op_res = &var_value != expected_value;
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    "ifGreaterThan" => {
                        let rule_value = test_case.ifGreaterThan.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let exp_val = match self.find_var(
                                        &expected_value,
                                        test_case,
                                        &fragment,
                                        &element,
                                    ) {
                                        Some(v) => v,
                                        None => expected_value.to_string(),
                                    };
                                    let actual: u8 = var_value.parse().unwrap();
                                    let expected: u8 = exp_val.parse().unwrap();
                                    let test_op_res = actual > expected;
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    "ifLessThan" => {
                        let rule_value = test_case.ifLessThan.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let exp_val = match self.find_var(
                                        &expected_value,
                                        test_case,
                                        &fragment,
                                        &element,
                                    ) {
                                        Some(v) => v,
                                        None => expected_value.to_string(),
                                    };
                                    let actual: u8 = var_value.parse().unwrap();
                                    let expected: u8 = exp_val.parse().unwrap();
                                    let test_op_res = actual < expected;
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    "ifGreaterThanOrEquals" => {
                        let rule_value = test_case.ifGreaterThanOrEquals.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let exp_val = match self.find_var(
                                        &expected_value,
                                        test_case,
                                        &fragment,
                                        &element,
                                    ) {
                                        Some(v) => v,
                                        None => expected_value.to_string(),
                                    };
                                    let actual: u8 = var_value.parse().unwrap();
                                    let expected: u8 = exp_val.parse().unwrap();
                                    let test_op_res = actual >= expected;
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    "ifLessThanOrEquals" => {
                        let rule_value = test_case.ifLessThanOrEquals.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let exp_val = match self.find_var(
                                        &expected_value,
                                        test_case,
                                        &fragment,
                                        &element,
                                    ) {
                                        Some(v) => v,
                                        None => expected_value.to_string(),
                                    };
                                    let actual: u8 = var_value.parse().unwrap();
                                    let expected: u8 = exp_val.parse().unwrap();
                                    let test_op_res = actual <= expected;
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    "ifNull" => {
                        let var_name = test_case.ifNull.as_ref().unwrap();
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            let var_res =
                                match self.find_var(&var_name, test_case, &fragment, &element) {
                                    Some(v) => v,
                                    None => NULL.to_owned(),
                                };
                            if var_res == NULL {
                                match self.assertions_ops(&fragment, &element, test_case, &fields) {
                                    Some(val) => {
                                        if val {
                                            res = OpsResult {
                                                name: test_case.name.to_string(),
                                                test_op: passed.to_owned(),
                                                assertion_op: passed.to_owned(),
                                            };
                                            test_results.push(res);
                                            continue;
                                        }
                                        res = OpsResult {
                                            name: test_case.name.to_string(),
                                            test_op: passed.to_owned(),
                                            assertion_op: failed.to_owned(),
                                        };
                                        test_results.push(res);
                                        continue;
                                    }
                                    None => {
                                        res = OpsResult {
                                            name: test_case.name.to_string(),
                                            test_op: passed.to_owned(),
                                            assertion_op: nothing.to_owned(),
                                        };
                                        test_results.push(res);
                                        continue;
                                    }
                                }
                            }
                            res = OpsResult {
                                name: test_case.name.to_string(),
                                test_op: nothing.to_owned(),
                                assertion_op: nothing.to_owned(),
                            };
                            test_results.push(res);
                            continue;
                        }
                    }
                    "ifNotNull" => {
                        let var_name = test_case.ifNotNull.as_ref().unwrap();
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            let var_res =
                                match self.find_var(&var_name, test_case, &fragment, &element) {
                                    Some(v) => v,
                                    None => NULL.to_owned(),
                                };
                            if var_res != NULL {
                                match self.assertions_ops(&fragment, &element, test_case, &fields) {
                                    Some(val) => {
                                        if val {
                                            res = OpsResult {
                                                name: test_case.name.to_string(),
                                                test_op: passed.to_owned(),
                                                assertion_op: passed.to_owned(),
                                            };
                                            test_results.push(res);
                                            continue;
                                        }
                                        res = OpsResult {
                                            name: test_case.name.to_string(),
                                            test_op: passed.to_owned(),
                                            assertion_op: failed.to_owned(),
                                        };
                                        test_results.push(res);
                                        continue;
                                    }
                                    None => {
                                        res = OpsResult {
                                            name: test_case.name.to_string(),
                                            test_op: passed.to_owned(),
                                            assertion_op: nothing.to_owned(),
                                        };
                                        test_results.push(res);
                                        continue;
                                    }
                                }
                            }
                            res = OpsResult {
                                name: test_case.name.to_string(),
                                test_op: nothing.to_owned(),
                                assertion_op: nothing.to_owned(),
                            };
                            test_results.push(res);
                            continue;
                        }
                    }
                    "ifIncludes" => {
                        let rule_value = test_case.ifLessThanOrEquals.as_ref().unwrap();
                        let var_name = &rule_value[0];
                        let expected_value = &rule_value[1];
                        for element in fragment.select(&selector) {
                            let res: OpsResult;
                            match self.find_var(&var_name, test_case, &fragment, &element) {
                                Some(var_value) => {
                                    let test_op_res = var_value.contains(expected_value);
                                    if test_op_res {
                                        match self
                                            .assertions_ops(&fragment, &element, test_case, &fields)
                                        {
                                            Some(val) => {
                                                if val {
                                                    res = OpsResult {
                                                        name: test_case.name.to_string(),
                                                        test_op: passed.to_owned(),
                                                        assertion_op: passed.to_owned(),
                                                    };
                                                    test_results.push(res);
                                                    continue;
                                                }
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: failed.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                            None => {
                                                res = OpsResult {
                                                    name: test_case.name.to_string(),
                                                    test_op: passed.to_owned(),
                                                    assertion_op: nothing.to_owned(),
                                                };
                                                test_results.push(res);
                                                continue;
                                            }
                                        }
                                    }
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: failed.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                                None => {
                                    res = OpsResult {
                                        name: test_case.name.to_string(),
                                        test_op: nothing.to_owned(),
                                        assertion_op: nothing.to_owned(),
                                    };
                                    test_results.push(res);
                                    continue;
                                }
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
        return test_results;
    }

    fn assertions_ops<'a>(
        &self,
        fragment: &Html,
        element: &ElementRef,
        test_case: &'a mut &TestValue,
        fields: &Vec<&str>,
    ) -> Option<bool> {
        let null = NULL.to_owned();
        let mut res: Option<bool> = None;
        for &field in fields.iter() {
            match field {
                "assert" => {
                    let ref assertion_value = test_case.assert.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let result = test_fns::test_equals(&var_value, &expected_value);
                    res = Some(result);
                }
                "assertEquals" => {
                    let ref assertion_value = test_case.assertEquals.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let result = test_fns::test_equals(&var_value, &expected_value);
                    res = Some(result);
                }
                "assertNotEquals" => {
                    let ref assertion_value = test_case.assertNotEquals.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let result = test_fns::test_equals(&var_value, &expected_value);
                    res = Some(!result);
                }
                "assertGreaterThan" => {
                    let ref assertion_value = test_case.assertGreaterThan.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let result = test_fns::test_greater_than(&var_value, &expected_value);
                    res = Some(result);
                }
                "assertLessThan" => {
                    let ref assertion_value = test_case.assertLessThan.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let result = test_fns::test_less_than(&var_value, &expected_value);
                    res = Some(result);
                }
                "assertGreaterThanOrEquals" => {
                    let ref assertion_value = test_case.assertGreaterThanOrEquals.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let eq = test_fns::test_equals(&var_value, &expected_value);
                    let gt = test_fns::test_greater_than(&var_name, expected_value);
                    if eq || gt {
                        res = Some(true);
                    } else {
                        res = Some(false);
                    }
                }
                "assertNotGreaterThan" => {
                    let ref assertion_value = test_case.assertNotGreaterThan.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let gt = test_fns::test_greater_than(&var_value, expected_value);
                    if gt {
                        res = Some(false);
                    } else {
                        res = Some(true);
                    }
                }
                "assertLessThanOrEquals" => {
                    let ref assertion_value = test_case.assertLessThanOrEquals.as_ref().unwrap();
                    let ref var_name = &assertion_value[0];
                    let ref expected_value = &assertion_value[1];
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let eq = test_fns::test_equals(&var_value, &expected_value);
                    let lt = test_fns::test_less_than(&var_value, expected_value);
                    if eq || lt {
                        res = Some(true);
                    } else {
                        res = Some(false);
                    }
                }
                "assertNull" => {
                    let ref var_name = test_case.assertNull.as_ref().unwrap();
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let result = test_fns::test_equals(&var_value, &String::from(NULL));
                    res = Some(result);
                }
                "assertNotNull" => {
                    let ref var_name = test_case.assertNotNull.as_ref().unwrap();
                    let var_value = match self.find_var(&var_name, test_case, &fragment, &element) {
                        Some(v) => v,
                        None => String::from(NULL),
                    };
                    let result = test_fns::test_equals(&var_value, &null);
                    res = Some(!result);
                }
                _ => res = None,
            }
        }
        return res;
    }

    // finds rule specs default and defined variables
    fn find_var<'a>(
        &self,
        spec_expr: &str,
        test_case: &'a mut &TestValue,
        fragment: &Html,
        element: &ElementRef,
    ) -> Option<String> {
        let contains_rlet = test_case.non_none_fields().contains(&"r#let");
        let contains_let = test_case.non_none_fields().contains(&"let");
        let default_vars: [&str; 4] = ["$element", "$count", "$innerText", "$attributes"];
        let eq = "=";
        // &element
        if &spec_expr == &default_vars[0] {
            let el = element.value().name().to_owned();
            return Some(el);
        }
        // &innerText
        if &spec_expr == &default_vars[2] {
            return Some(self.innerText(element).to_string());
        }
        // &count
        if spec_expr.contains(&default_vars[1]) {
            // $count{*[id="$attributes[aria-labelledby]"]}
            let mut query = spec_expr
                .replace(&default_vars[1], "")
                .replace(&['{', '}', '"'][..], "");
            // *[id=$attributes[aria-labelledby]]
            let is_all_query = query.contains("*");
            if is_all_query {
                &query.remove(0);
                &query.remove(0);
                &query.pop();
            } else {
                &query.remove(0);
                &query.pop();
            }
            let counter: &mut u8 = &mut 0u8;
            // "innerText" ex. $count{*[innerText=$innerText]}
            let is_inner_text_query = query.contains(default_vars[2].to_owned().remove(0));
            let eq_index = &query.find(eq).unwrap();
            let mut query_arg_expr = query.replace(eq, "");
            let second = query_arg_expr.split_off(*eq_index); // this should be $innerText, attributes[somename] or some defined variable with let
            if is_inner_text_query {
                // At this point second should be == $innerText or some defined variable with let
                let inner_text_match = if second == default_vars[2] {
                    self.innerText(element)
                } else if contains_rlet || contains_let {
                    let spec_let = test_case.r#let.as_ref().unwrap();
                    if spec_let.contains_key(&second) {
                        let v = match spec_let.get(&second) {
                            Some(v) => {
                                let nw = v.to_owned();
                                nw
                            }
                            None => {
                                let n = "null".to_owned().clone();
                                n
                            }
                        };
                        v
                    } else {
                        let v = match self.find_var(&second, test_case, &fragment, &element) {
                            Some(v) => v.to_owned(),
                            None => {
                                let n = NULL.to_owned().clone();
                                n
                            }
                        };
                        v
                    }
                } else {
                    let v = match self.find_var(&second, test_case, &fragment, &element) {
                        Some(v) => v.to_owned(),
                        None => {
                            let n = NULL.to_owned().clone();
                            n
                        }
                    };
                    v
                };
                if is_all_query {
                    let root_element = fragment.root_element();
                    self.count_all_inner_text_match(counter, &root_element, &inner_text_match);
                    return Some(counter.to_string());
                }
                if *self.innerText(&element) == inner_text_match.clone() {
                    let n = 1u8;
                    *counter = *counter + n;
                }
                return Some(counter.to_string());
            }

            // "attributes" ex. $count{*[id="$attributes[aria-labelledby]"]}
            let is_attributes_query = query.contains(default_vars[3].to_owned().remove(0));
            if is_attributes_query {
                let attribute_value = if second.contains(default_vars[3]) {
                    self.attributes(&second, &element).to_owned()
                } else if contains_rlet || contains_let {
                    let spec_let = test_case.r#let.as_ref().unwrap();
                    if spec_let.contains_key(&second) {
                        let v = match spec_let.get(&second) {
                            Some(v) => String::from(v),
                            None => String::from(NULL),
                        };
                        v
                    } else {
                        let v = match self.find_var(&second, test_case, &fragment, &element) {
                            Some(v) => v.to_owned(),
                            None => String::from(NULL),
                        };
                        v 
                    }
                } else {
                    let v = match self.find_var(&second, test_case, &fragment, &element) {
                        Some(v) => v.to_owned(),
                        None => String::from(NULL),
                    };
                    v
                }
                .to_string();
                // let query = spec_expr.replace(&default_vars[1], "").replace(&['{', '}', '"'][..],"");
                // query = *[id=$attributes[aria-labelledby]]
                let _ = query.split_off(*eq_index + 1); //$attributes[aria-labelledby]]
                let new_query = query + &attribute_value + &"]".to_owned(); // *[id= + a + attribute_value + "]"
                let s = Selector::parse(&new_query).unwrap();
                let elems = fragment.select(&s);
                return Some(elems.count().to_string());
            }
            return Some(counter.to_string());
        }

        // attributes
        if spec_expr.contains(&default_vars[3]) {
            // let eq_index = &spec_expr.find(eq).unwrap();
            // let mut expr = spec_expr.replace(eq, "");

            // let attrib = spec_expr
            //     .replace(&default_vars[3], "")
            //     .replace(&['[', ']'][..], "");
            return Some(self.attributes(&spec_expr, element).to_owned());
        }

        // let
        let key = &spec_expr.to_owned();
        if contains_rlet || contains_let {
            let spec_let = test_case.r#let.as_ref().unwrap();
            if spec_let.contains_key(key) {
                return match spec_let.get(key) {
                    Some(v) => self.find_var(v, test_case, &fragment, &element),
                    None => None,
                };
            }
        }
        None
    }

    fn innerText(&self, element: &ElementRef) -> String {
        element.text().collect::<String>()
    }

    // Retrieves attribute
    fn attributes<'a>(&self, spec_expr: &str, element: &ElementRef<'a>) -> &'a str {
        let attribute_name = spec_expr
            .replace("$attributes", "")
            .replace(&['[', ']'][..], "");
        match element.value().attr(&attribute_name) {
            Some(v) => v,
            None => NULL,
        }
    }

    /// Counts all matching innerTexts with an increment counter and innerText matcher
    fn count_all_inner_text_match(
        &self,
        counter: &mut u8,
        element: &ElementRef,
        inner_text_match: &String,
    ) {
        let elem_inner_text = self.innerText(&element);
        if &elem_inner_text == inner_text_match {
            let n = 1u8;
            *counter = *counter + n;
        }
        if element.has_children() {
            match &element.first_child() {
                Some(node_ref) => {
                    let nxt_el = ElementRef::wrap(*node_ref).unwrap();
                    self.count_all_inner_text_match(counter, &nxt_el, &inner_text_match)
                }
                None => (),
            }
        }
        if element.has_siblings() {
            match &element.next_sibling() {
                Some(node_ref) => {
                    let nxt_el = ElementRef::wrap(*node_ref).unwrap();
                    self.count_all_inner_text_match(counter, &nxt_el, &inner_text_match)
                }
                None => (),
            }
        }
    }
}

fn default_hidden() -> bool {
    false
}
