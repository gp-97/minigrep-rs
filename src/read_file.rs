use crate::config::Config;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn get_file_contents(path: &String) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut content = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn extract_content(config: &Config, content_box: &mut String) {
    let contents = get_file_contents((*config).get_filename()).unwrap_or_else(|err| {
        eprintln!(
            "[ERROR]: {}\n\tProvide full filepath with correct filename",
            err
        );
        process::exit(1);
    });
    content_box.push_str(contents.as_str());
}

pub fn run(config: &Config, content_box: &mut String) {
    extract_content(config, content_box);
}
