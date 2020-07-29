use std::string::String;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use smol;
use surf;
use anyhow::{Error};
use scraper::{Html, Selector};
mod rules_spec;
mod test_fns;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "digr",
    about = "an automated accessibility test runner based on rules"
)]
struct Arguments {
    #[structopt(short = "r", long = "rules", help = "Rules folder")]
    rules: PathBuf,

    #[structopt(short = "u", long = "url", help = "Url to test")]
	url: String,

    #[structopt(short = "d", long = "depth", help = " Depth or resources to follow on page", default_value = "0")]	
	depth: u8,
}

fn main() {
    let opts = Arguments::from_args();
    let site_url: &str = &opts.url;

    let file = File::open(opts.rules).expect("Unable to open file, please remember file or folder argument with the -f option.");
    let spec: rules_spec::RuleSpec = serde_yaml::from_reader(file).expect("There was an error parsing rules file.");
	
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
        
        for tag in spec.on.iter() {
            let selector = match Selector::parse(tag) {
                Ok(s) => s,
                Err(_) => Selector::parse("h3.nil").unwrap() // this isn't ideal but a work around
            };
            let test_result = spec.rules_ops(&selector, &fragment);
            for res_op in test_result.iter() {
                println!("{:}", res_op);
            }
        }
	});

}


