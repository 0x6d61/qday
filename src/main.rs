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
        flag_t: bool,
        flag_help: bool,
    }

    const USAGE: &'static str = 
"
Usage:
    qday <tag>
    qday <tag> -t <time>
    qday --help

Options:
    -h --help     Show this screen.
    -t            date
";

    let args: Args = Docopt::new(USAGE).and_then(|d|d.deserialize())
        .unwrap_or_else(|e|e.exit());

    let date = match args.flag_t {
        true => args.arg_time,
        false => time::strftime("%Y-%m-%d",&time::now()).unwrap(),
    };

    let url = format!("https://qiita.com/api/v2/items?page=1&query=tag:{}+created:>{}",args.arg_tag,date);
    let resp = reqwest::get(&url).unwrap().text().unwrap();
    let topick_json: Value = serde_json::from_str(&resp).unwrap();
    let topick_array = &topick_json.as_array().unwrap();
    for i in topick_array.iter() {
        println!("Title: {title:}\nUrl: {url:}",title=i["title"].as_str().unwrap(),url=i["url"].as_str().unwrap());
    }
}










