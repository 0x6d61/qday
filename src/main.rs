extern crate serde_json;
extern crate serde;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate colored;
extern crate time;

use docopt::Docopt;
use serde_json::Value;
use colored::*;


fn main() {

    #[derive(Debug,Deserialize)]
    struct Args  {
        arg_tag: String,
        arg_time: String,
        flag_d: bool,
        flag_help: bool,
		flag_t:bool,
    }

    const USAGE: &'static str =
"
Usage:
    qday <tag>
    qday <tag> -d <time>
    qday <tag> -t -d <time>
    qday --help

Options:
    -h --help     Show this screen.
    -d            date
	-t            title
";

    let args: Args = Docopt::new(USAGE).and_then(|d|d.deserialize())
        .unwrap_or_else(|e|e.exit());

    let date = match args.flag_d {
        true => args.arg_time,
        false => time::strftime("%Y-%m-%d",&time::now()).unwrap(),
    };

    let url = format!("https://qiita.com/api/v2/items?page=1&query=tag:{}+created:>{}",args.arg_tag,date);
    let resp = reqwest::get(&url).unwrap().text().unwrap();
    let topick_json: Value = serde_json::from_str(&resp).unwrap();
    let topick_array = &topick_json.as_array().unwrap();
    for i in topick_array.iter() {
		if !args.flag_t {
				println!("{}",i["url"].as_str().unwrap());
		}else{
        	println!("Title: {title:}\nUrl: {url:}",title=i["title"].as_str().unwrap(),url=i["url"].as_str().unwrap());
		}
	}
}
