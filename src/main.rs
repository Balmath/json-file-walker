mod config;

use config::Config;
use json_file_walker::walk_json_files;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    for path in walk_json_files(config.root_dir) {
        println!("{}", path.to_string_lossy());
    }
}