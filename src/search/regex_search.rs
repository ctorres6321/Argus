use regex::Regex;

pub fn matches(line: &str, query: &Regex) -> bool {
    query.is_match(line)
}