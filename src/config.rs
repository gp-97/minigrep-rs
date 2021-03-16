#[derive(Debug)]
pub struct Config {
    filename: String,
    query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() <= 2 {
            Err(String::from("not enough arguments"))
        } else {
            Ok(Self {
                filename: String::from(&args[1]),
                query: String::from(&args[2]),
            })
        }
    }
    pub fn get_filename(&self) -> &String {
        &self.filename
    }
    pub fn get_query(&self) -> &String {
        &self.query
    }
}
