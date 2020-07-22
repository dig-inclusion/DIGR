use crate::test_fns;
use non_none_fields::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::string::String;
use scraper::{Html, Selector, ElementRef};

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

impl RuleSpec {
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
                        // if Some(true) =
                        //     test_fns::test_equals(&current_value, &expected_value, &elem)
                        // {
                            let ref assertion_value = fields[2];

                            match self.assertions_ops(assertion_value, index, &elem) {
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
                                },
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
                    Some(false) => Some(true),
                }
            }
            _ => None,
        }
    }

    // finds rule specs default and defined variables
    fn find_var(&self, spec_expr: &str, index: usize, fragment: &Html, element: &ElementRef,  selector: &Selector) -> Option<&String> {
        let ref test = &self.tests[index];
        let test_let = &test.r#let.unwrap();
        let default_vars: [&str;4] = ["$element", "$count", "$innerText", "$attributes"];
        let eq = "=";
        // &element
        if &spec_expr == &default_vars[0] {
            return Some(&element.value().name().to_owned())
        }
        // &innerText
        if &spec_expr == &default_vars[2] {
            return Some(self.innerText(element));
        }
        // &count
        if spec_expr.contains(&default_vars[1]) {
            // $count{*[id="$attributes[aria-labelledby]"]}
            let query = spec_expr.replace(&default_vars[1], "").replace(&['{', '}', '"'][..],"");
            // *[id=$attributes[aria-labelledby]]
            let is_all_query =  query.contains("*");
            if is_all_query {
                &query.remove(0);
                &query.remove(0);
                &query.pop();
            } else {
                &query.remove(0);
                &query.pop();
            }
            let mut counter: u8 = 0;
            // "innerText" ex. $count{*[innerText=$innerText]}
            let is_inner_text_query =  query.contains(default_vars[2].to_owned().remove(0));
            let eq_index = &query.find(eq).unwrap();
            let mut query_arg_expr = query.replace(eq,"");
            let second = query_arg_expr.split_off(*eq_index); // this should be $innerText, attributes[somename] or some defined variable with let
            if is_inner_text_query {
                // At this point second should be == $innerText or some defined variable with let
                let inner_text_match = if second == default_vars[2] { 
                                    self.innerText(element)
                                 } else if test_let.contains_key(&second) { 
                                    match &test_let.get(&second) {
                                        Some(v) => v,
                                        None => &"NULL".to_owned()
                                    } 
                                 } else { 
                                    match self.find_var(&second, index, &fragment, &element, &selector) {
                                        Some(v) => v,
                                        None => &"NULL".to_owned()
                                    }
                                 };
                if is_all_query {
                    let root_element = fragment.root_element();
                    self.count_all_inner_text_match(&counter, &root_element, inner_text_match);
                    return Some(&counter.to_string());
                }
                for element in fragment.select(&selector) {
                    if self.innerText(&element) == inner_text_match {
                        counter = counter + 1;  
                    }
                }
                return Some(&counter.to_string());
            }

            // "attributes" ex. $count{*[id="$attributes[aria-labelledby]"]}
            let is_attributes_query = query.contains(default_vars[3].to_owned().remove(0));
            if is_attributes_query {
                let attribute_value = if second.contains(default_vars[3]) { 
                                            self.attributes(&second, &element)
                                        } else if test_let.contains_key(&second) { 
                                            match &test_let.get(&second) {
                                                Some(v) => v,
                                                None => &"null".to_owned()
                                            } 
                                        } else { 
                                            match self.find_var(&second, index, &fragment, &element, &selector) {
                                                Some(v) => v,
                                                None => &"null".to_owned()
                                            }
                                        };
                // let query = spec_expr.replace(&default_vars[1], "").replace(&['{', '}', '"'][..],"");
                // query = *[id=$attributes[aria-labelledby]]
                let _ = query.split_off(*eq_index + 1); //$attributes[aria-labelledby]]
                let new_query = query + attribute_value + "]"; // *[id= + a + attribute_value + "]"
                let s = Selector::parse(&new_query).unwrap();
                let elems = fragment.select(&s);
                return Some(&elems.count().to_string());
            }
            return Some(&counter.to_string());
        }

        // attributes
        if spec_expr.contains(&default_vars[3]){
            let eq_index = &spec_expr.find(eq).unwrap();
            let mut expr = &spec_expr.replace(eq,"");
            let attrib = expr.split_off(*eq_index).replace(&default_vars[3],"").replace(&['[', ']'][..],"");
            return Some(&self.attributes(&attrib, element).to_owned());
        }
        
        // let
        let key = &spec_expr.to_owned();
        if test_let.contains_key(key) {
            return match test_let.get(key) {
                Some(v) => self.find_var(v, index, &fragment, &element, &selector),
                None => None
            };
        }
        None
    }
    fn innerText(&self, element: &ElementRef) -> &String {
        &element.text().collect::<String>()
    }
    // Retrieves attribute
    fn attributes<'a>(&self, spec_expr: &str, element: &ElementRef<'a>) -> &'a str {
        let attribute_name = spec_expr.replace("$attributes","").replace(&['[', ']'][..],"");
        match element.value().attr(&attribute_name) {
            Some(v) => v,
            None => "NULL"
        }
    }

    /// Counts all matching innerTexts with an increment counter and innerText matcher
    fn count_all_inner_text_match(&self, counter: &u8, element: &ElementRef, inner_text_match: &String) {
        let elem_inner_text = self.innerText(&element);
        if elem_inner_text == inner_text_match {
            *counter = *counter + 1;  
        }
        if element.has_children(){
            match &element.first_child() {
                Some(node_ref) => self.count_all_inner_text_match(&counter, &ElementRef { node: *node_ref }, &inner_text_match),
                None => _,
            }
        }
        if element.has_siblings(){
            match &element.next_sibling() {
                Some(node_ref) => self.count_all_inner_text_match(&counter, &ElementRef { node: *node_ref }, &inner_text_match),
                None => _,
            }
        }
    }
}

fn default_hidden() -> bool {
    false
}
