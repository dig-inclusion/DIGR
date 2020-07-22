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


// Function for html tags operations
fn tag_ops(fragment: &Html, tag: &std::string::String) {

    let selector = Selector::parse(&tag).expect("Invalid CSS selector.");
    // for element in fragment.select(&selector) {

    //     println!("{:?}", element.value().name);

        
    // }
    fragment.select(&selector).for_each(|element| { 
            let name = &element.value().name;
            print!("{:?}", name);
        });
}

//
// Function for attributes operations
fn attr_ops(index: usize, fragment: &Html, tag: &std::string::String) {

    let tg = tag.replace(&['(', '*', ']', '\''][..], "");
    
    let (first, last) = tg.split_at(index);
    
    let mut attrib = first.to_string();
    attrib.push(']');

    let l = &last.replace('=', "");
    let attr_name: &str = &l;

    let selector = Selector::parse(&tg).expect("Invalid CSS selector.");
    // for element in fragment.select(&selector) {

    //     for elem in &element.value().attr(attr_name) {
    //         println!("{:?}", elem);

    //     }
        
    // }

    fragment.select(&selector).for_each(|element| { 
            let elem = element.value().attr(attr_name).unwrap();
            print!("{:?}", elem);
        });
}

fn main() {
    let opts = Arguments::from_args();
    let site_url: &str = &opts.url;

    // println!("{} \n {:?}", site_url, opts);

    let file = File::open(opts.rules).expect("Unable to open file, please remember file or folder argument with the -f option.");

    let spec: rules_spec::RuleSpec = serde_yaml::from_reader(file).expect("There was an error parsing RuleSpec");

	// println!("{:?}", spec);
	
	smol::run(async {

		let body = surf::get(site_url)
			.recv_string().await
            .map_err(Error::msg);

		// println!("Site html: {:?}", body);

		let b = match body {
			Ok(html) => html,
			Err(error) => panic!("Problem accessing the url: {:?}", error),
		};

		let page_slice: &str = &b;
        let fragment = Html::parse_fragment(page_slice);
        
        for tag in spec.on.iter() {

            let is_index = tag.find("=");

            match is_index {
                Some(index) => attr_ops(index, &fragment, tag),
                None => tag_ops(&fragment, tag),
            };

        }
	});

}


