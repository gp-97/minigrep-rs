pub mod config;
pub mod read_file;
pub mod search;
use config::Config;
use read_file::run;
use std::env;
use std::process;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args[1].split("/").collect::<Vec<&str>>().len() < 3 {
        let path_buf = match env::current_dir() {
            Ok(p) => p,
            Err(err) => {
                eprintln!("[ERROR]: {}", err);
                process::exit(1);
            }
        };
        let path = match path_buf.as_os_str().to_str() {
            Some(p) => p,
            None => {
                eprintln!("[ERROR]: No such file exists in the current directory");
                process::exit(1);
            }
        };
        let mut temp_file_path = String::new();
        temp_file_path.push_str(path);
        temp_file_path.push('/');
        temp_file_path.push_str(&args[1]);
        args[1] = String::from(&*temp_file_path);
    }
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("[ERROR]: {}", err);
        process::exit(1);
    });

    let mut content_box = String::new();
    run(&config, &mut content_box);
    search::search_query(&config, &content_box);
}
