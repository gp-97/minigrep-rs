pub mod config;
pub mod read_file;
pub mod search;
use config::Config;
use read_file::run;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("[ERROR]: {}", err);
        process::exit(1);
    });

    let mut content_box = String::new();
    run(&config, &mut content_box);
    search::search(&config, &content_box);
}
