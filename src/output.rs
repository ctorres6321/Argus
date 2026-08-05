use colored::Colorize;

pub struct SearchResult{
    pub path: String,

    // Everything we want to print
    pub context: Vec<(usize, String)>,

    // Which line matches
    pub match_line: usize,
}

pub fn print_match(result: &SearchResult){
    println!("{}", result.path.blue());
    
    for(line_number, line) in &result.context{
        if *line_number == result.match_line {
            println!("{}{} | {}", ">".red().bold(), line_number.to_string().green(), line.bold());
        } else {
            println!("{} | {}", line_number.to_string().dimmed(), line.dimmed(),); 
        }
    }
    println!();
}
