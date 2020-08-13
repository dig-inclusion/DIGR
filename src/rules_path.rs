use std::result{Error, Result};
use std::string::String;
use std::fs::{File, read_dir};
use std::path::PathBuf;
use scraper::{Html, Selector};
use crate::rules_spec;

/// Sends a request and fetches the response.
pub async fn file_op(rules_file: PathBuf, tags: &Vec<String>, fragment: &Html) -> Result<Vec<rules_spec::OpsResult>, Error> {
    let file = File::open(rules_file).expect("Unable to open file, please remember file or folder argument with the -f option.");
    let spec: rules_spec::RuleSpec = serde_yaml::from_reader(file).expect("There was an error parsing rules file.");
    let mut test_results: Vec<rules_spec::OpsResult> = vec![];
    for tag in tags.iter() {
        let s = "h3.nil"; // this isn't ideal but a work around
        let selector = match Selector::parse(tag) {
            Ok(s) => s,
            Err(_) => Selector::parse(s).unwrap() 
        };
        let op_res = spec.rules_ops(&selector, &fragment);
        test_results.extend(op_res);
        // println!("{}", tag);
        // for res_op in test_result.iter() {
        //     println!("{:?}", res_op);
        // }
    }
    return Ok(test_results);
}

/// Sends a request and fetches the response.
pub async fn folder_op<'a>(rules_folder: PathBuf, tags: &Vec<String>, fragment: &Html) -> Result<Vec<rules_spec::OpsResult>, Error> {
    let mut test_results: Vec<rules_spec::OpsResult>;
    for entry in read_dir(rules_folder)? {
        let entry = entry?;
        let path = entry.path();
        let res = file_op(path, &tags, &fragment).await;
        test_results.extend(res);
    }
    return Ok(test_results);
}

