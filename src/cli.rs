use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {

    /// Text to search for
    pub query: String,

    /// Directory to search
    pub path: String,

    /// Ignore case distinctions
    #[arg(short = 'l', long)]
    pub ignore_case: bool, 

    /// Enable regex support
    #[arg(short = 'r', long)]
    pub enable_regex: bool, 

    /// Number of lines to search for after 
    after: Option<usize>,

    /// Number of lines to search for before 
    pub before: Option<usize>,

    /// Number of lines to search for context 
    #[arg(short = 'c', default_value_t = 0)]
    pub context: usize,

//  #[arg(short = 'a', long)]
//  pub enable_context_after: bool, 

    // Enables context for pattern matching (Prints N number of lines before the pattern was found)
//  #[arg(short = 'b', long)]
//  pub enable_context_before: bool, 

}
