pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    search_lazy(query, content).collect()
}

pub fn search_case_insens<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    search_case_insens_lazy(query, content).collect()
}

pub fn search_lazy<'a>(query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    content.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insens_lazy<'a>(query: &str, content: &'a str) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();
    let contains = move |line: &&str| line.to_lowercase().contains(&query);
    content.lines().filter(contains)
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

    #[test]
    fn lazy() {
        let query = "you";
        let mut matches = search_lazy(query, POEM);
        assert_eq!("I'm nobody! Who are you?", matches.next().unwrap());
        assert_eq!("Are you nobody, too?", matches.next().unwrap());
        assert_eq!("They'd banish us, you know.", matches.next().unwrap());
    }

    #[test]
    fn lazy_case_sens() {
        let query = "BaNiSh";
        assert_eq!(0, search_lazy(query, POEM).count());
    }

    #[test]
    fn lazy_case_insens() {
        let query = "aRe";
        let mut matches = search_case_insens_lazy(query, POEM);
        assert_eq!("I'm nobody! Who are you?", matches.next().unwrap());
        assert_eq!("Are you nobody, too?", matches.next().unwrap())
    }
}
