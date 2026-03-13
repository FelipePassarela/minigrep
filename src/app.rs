use crate::config;

use std::{error::Error, fs};

pub fn search<'a>(query: &str, content: &'a str, case_sens: bool) -> Vec<&'a str> {
    let mut result = vec![];

    let query = if case_sens {
        &query.to_lowercase()
    } else {
        query
    };

    for line in content.lines() {
        let normalized_line = if case_sens { &line.to_lowercase() } else { line };

        if normalized_line.contains(query) {
            result.push(line.trim());
        }
    }

    result
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content, false) {
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
        assert_eq!(result, search(query, POEM, false));
    }

    #[test]
    fn multiple_results() {
        let query = "no";
        let result = vec![
            "I'm nobody! Who are you?",
            "Are you nobody, too?",
            "They'd banish us, you know.",
        ];
        assert_eq!(result, search(query, POEM, false));
    }

    #[test]
    fn no_result() {
        let query = "monophormization";
        assert!(search(query, POEM, false).is_empty());
    }

    #[test]
    fn case_sensitive() {
        let query = "aRe";
        assert!(search(query, POEM, false).is_empty());
    }

    #[test]
    fn case_insensitive() {
        let query = "aRe";
        let result = vec!["I'm nobody! Who are you?", "Are you nobody, too?"];
        assert_eq!(result, search(query, POEM, true));
    }
}
