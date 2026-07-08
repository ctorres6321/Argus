pub struct SearchResult{
    pub path: String,
    pub line_number: usize,
    pub line: String,
}

pub fn print_match(result: &SearchResult){
    println!("{}:{}: {}", result.path, result.line_number, result.line);
}