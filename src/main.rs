#[macro_use]
extern crate serde;
extern crate serde_yaml;

use std::collections::BTreeMap;
use structopt::StructOpt;
use std::path::{PathBuf};
use std::fs::File;

#[derive(StructOpt, Debug)]
#[structopt(name = "digr", about = "an automated accessibility test runner based on rules")]
struct Arguments {
	#[structopt(short = "r", long = "rules", help = "Rules folder")]
	rules: PathBuf,

	#[structopt(short = "u", long = "url", help = "Url to test")]
	url: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct TestValue {
    name: String,
    r#if: Option<Vec<String>>, 
    r#let: Option<String>,
    links: Option<String>,
    ifNotEquals: Option<Vec<String>>,
    ifNotNull: Option<String>,
    assertNotGreaterThan: Option<Vec<String>>,
    assertNotEquals: Option<Vec<String>>,
    assertNotNull: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct ValidationValue {
    name: String,
    case: String,
    assert: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct RuleSpec {
    name: String,
    meta: Option<BTreeMap<String, String>>,
	on: Vec<String>,
	#[serde(default="default_hidden")]
    includeHidden: bool,
    tests: Vec<TestValue>,
    validation: Vec<ValidationValue>
}

fn default_hidden() -> bool {
	false
}

fn main() -> Result<(), serde_yaml::Error> {
	let opts = Arguments::from_args();
	println!("{:?}", opts);
  
    let file = File::open(opts.rules).expect("Unable to open file");

   let s: RuleSpec = serde_yaml::from_reader(file).unwrap();

	println!("{:?}", s);
		Ok(())
}