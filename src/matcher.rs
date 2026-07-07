pub enum SearchMode{
    Literal(String),
    Regex(Regex),
}

impl SearchMode{
    pub fn matches(&self, line: &str) -> bool{
        match self{
            SearchMode::Literal(query) => line.contains(query),
            SearchMode::Regex(regex) => regex.is_match(line),
        }
    }
}