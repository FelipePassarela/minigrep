use crate::config;

use std::{error::Error, fs};

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insens<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let matches = if config.ignore_case {
        search_case_insens(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in matches {
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

    #[test]
    fn case_sensitive() {
        let query = "BaNiSh";
        assert!(search(query, POEM).is_empty());
    }

    #[test]
    fn case_insensitive() {
        let query = "aRe";
        let matches = vec!["I'm nobody! Who are you?", "Are you nobody, too?"];
        assert_eq!(matches, search_case_insens(query, POEM))
    }
}
