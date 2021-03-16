use crate::config::Config;

pub fn search(config: &Config, content: &String) {
    let query: &String = config.get_query();
    println!("{}", query.as_str());
    for (count, line) in content.lines().enumerate() {
        if line.contains(query.as_str()) {
            println!("Line {}: {}", count + 1, line);
        }
    }
}
