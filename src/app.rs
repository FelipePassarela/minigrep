use crate::config;

use std::{error::Error, fs};

#[derive(Debug, PartialEq)]
pub enum Case {
    Sensitive,
    Insensitive,
}

#[derive(Debug)]
pub struct SearchParams {
    pub case: Case,
}

pub fn search<'a>(query: &str, content: &'a str, params: SearchParams) -> Vec<&'a str> {
    let mut result = vec![];

    let case_sens = params.case == Case::Insensitive;
    let normalized_query = if case_sens {
        &query.to_lowercase()
    } else {
        query
    };

    for line in content.lines() {
        let normalized_line = if case_sens {
            &line.to_lowercase()
        } else {
            line
        };

        if normalized_line.contains(normalized_query) {
            result.push(line.trim());
        }
    }

    result
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let params = SearchParams {
        case: Case::Sensitive,
    };
    for line in search(&config.query, &content, params) {
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
        let params = SearchParams {
            case: Case::Insensitive,
        };
        let result = vec!["Then there's a pair of us - don't tell!"];
        assert_eq!(result, search(query, POEM, params));
    }

    #[test]
    fn multiple_results() {
        let query = "no";
        let params = SearchParams {
            case: Case::Insensitive,
        };
        let result = vec![
            "I'm nobody! Who are you?",
            "Are you nobody, too?",
            "They'd banish us, you know.",
        ];
        assert_eq!(result, search(query, POEM, params));
    }

    #[test]
    fn no_result() {
        let query = "monophormization";
        let params = SearchParams {
            case: Case::Insensitive,
        };
        assert!(search(query, POEM, params).is_empty());
    }

    #[test]
    fn case_sensitive() {
        let query = "aRe";
        let params = SearchParams {
            case: Case::Sensitive,
        };
        assert!(search(query, POEM, params).is_empty());
    }

    #[test]
    fn case_insensitive() {
        let query = "aRe";
        let params = SearchParams {
            case: Case::Insensitive,
        };
        let result = vec!["I'm nobody! Who are you?", "Are you nobody, too?"];
        assert_eq!(result, search(query, POEM, params));
    }
}
