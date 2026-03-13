use crate::config;

use std::{error::Error, fs};

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in content.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }

    result
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content) {
        println!("{line}")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static POEM: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

    #[test]
    fn one_result() {
        let query = "there";
        let result = vec!["Then there's a pair of us - don't tell!"];
        assert_eq!(result, search(query, POEM));
    }

    #[test]
    fn no_result() {
        let query = "monophormization";
        assert!(search(query, POEM).is_empty());
    }
}
