pub mod args_cleanup;
pub mod config;
pub mod read_file;
pub mod search;
use config::Config;
use read_file::run;
use std::env;
use std::process;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args_cleanup::clean(&mut args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("[ERROR]: {}", err);
        process::exit(1);
    });

    let mut content_box = String::new();
    run(&config, &mut content_box);
    search::search_query(&config, &content_box);
}
