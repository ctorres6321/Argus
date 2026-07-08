use colored::Colorize;

pub struct SearchResult{
    pub path: String,
    pub line_number: usize,
    pub line: String,
}

pub fn print_match(result: &SearchResult){
    println!("{}:{}| {}", result.path.yellow(), result.line_number.to_string().magenta() , result.line);
}