use crate::config;

use std::{error::Error, fs};

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matches = vec![];

    for line in content.lines() {
        if line.contains(query) {
            matches.push(line.trim());
        }
    }

    matches
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
    fn one_match() {
        let query = "there";
        let matches = vec!["Then there's a pair of us - don't tell!"];
        assert_eq!(matches, search(query, POEM));
    }

    #[test]
    fn multiple_matches() {
        let query = "you";
        let matches = vec![
            "I'm nobody! Who are you?",
            "Are you nobody, too?",
            "They'd banish us, you know.",
        ];
        assert_eq!(matches, search(query, POEM));
    }

    #[test]
    fn substring() {
        let query = "ban";
        let matches = vec!["They'd banish us, you know."];
        assert_eq!(matches, search(query, POEM))
    }

    #[test]
    fn no_match() {
        let query = "monophormization";
        assert!(search(query, POEM).is_empty());
    }
}
