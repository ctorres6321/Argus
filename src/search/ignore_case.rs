pub fn matches(line: &str, query: &str) -> bool {
    line.to_lowercase().contains(query)
}