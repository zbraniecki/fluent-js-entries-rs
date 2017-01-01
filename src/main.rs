extern crate serde;
extern crate serde_json;
extern crate fluent;
extern crate fluent_js_entries;
extern crate getopts;

use std::fs::File;
use std::io::Read;
use std::io;
use std::env;

use getopts::Options;

use fluent::syntax::parser::parse;

use fluent_js_entries::*;
use fluent_js_entries::serialize_json;

#[derive(Debug)]
enum CliError {
    Deserialize(serde_json::Error),
    Parse(fluent::syntax::parser::ParserError),
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn print_resource(res: &Resource) {
    println!("{:?}", res);
}

fn print_json_resource(res: &Resource) {
    println!("{}", serialize_json(&res));
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("s", "silence", "disable output");
    opts.optflag("r", "raw", "print raw result");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let source = read_file(&input).expect("Read file failed");
    let res: Result<Resource, CliError> = if input.contains(".json") {
        serde_json::from_str(&source).map_err(|err| CliError::Deserialize(err))
    } else {
        parse(&source).map(|res| Resource::from(res)).map_err(|err| CliError::Parse(err))
    };

    if matches.opt_present("s") {
        return;
    };

    match res {
        Ok(res) => {
            if matches.opt_present("r") {
                print_resource(&res);
            } else {
                print_json_resource(&res);
            }
        }
        Err(err) => println!("Error: {:?}", err),
    };
}
