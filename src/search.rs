use crate::config::Config;

pub fn search(config: &Config, content: &String) {
    let query: &String = config.get_query();
    let mut instances = 0;
    for (count, line) in content.lines().enumerate() {
        if line.contains(query.as_str()) {
            println!("Line {}: {}", count + 1, line);
            instances += 1;
        }
    }
    if instances == 0 {
        println!("No match found");
    }
}
