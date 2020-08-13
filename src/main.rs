use std::string::String;
use std::fs::File;
use std::path::PathBuf;
use std::result::Result::Err;
use std::fs::metadata;
use structopt::StructOpt;
use smol;
use surf;
use anyhow::{Error};
use scraper::{Html};
mod rules_spec;
mod rules_path;
mod test_fns;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "DIGR",
    about = "An automated accessibility test runner based on rules"
)]
struct Arguments {
    #[structopt(short = "r", long = "rules", help = "Rules file or folder")]
    rules: String,

    #[structopt(short = "u", long = "url", help = "Url to test")]
	url: String,

    #[structopt(short = "d", long = "depth", help = " Depth or resources to follow on page", default_value = "0")]	
	depth: u8,
}

fn main(){
    let opts = Arguments::from_args();
    let site_url: &str = &opts.url;

    let md = metadata(opts.rules.clone()).unwrap();
    let is_file = md.is_file();
    let is_dir = md.is_dir();

    let file = File::open(opts.rules).expect("Unable to open file, please remember file or folder argument with the -r option.");
    let spec: rules_spec::RuleSpec = serde_yaml::from_reader(file).expect("There was an error parsing rules file.");
    let r :&str = &opts.rules;

	smol::run(async {
		let body = surf::get(site_url)
			.recv_string().await
            .map_err(Error::msg);

		let b = match body {
			Ok(html) => html,
			Err(error) => panic!("Problem accessing the url: {:?}", error),
		};
		let page_slice: &str = &b;
        let fragment = Html::parse_fragment(page_slice);
        let rules_path = PathBuf::from(r);

        if is_file {
c           let test_result = rules_path::file_op(rules_path, &spec.on, &fragment).await;
            for res_op in test_result.iter() {
                println!("{:?}", res_op);
            }
        }

        if is_dir {
            let test_result = rules_path::folder_op(rules_path, &spec.on, &fragment).await;
            for res_op in test_result.iter() {
                println!("{:?}", res_op);
            }
        }
	});
}


