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
    #[arg(short = 'E', long)]
    pub enable_regex: bool, 

    /// Number of lines to search for after match
    #[arg(short = 'A', long)]
    pub after: Option<usize>,

    /// Number of lines to search for before 
    #[arg(short = 'B', long)]
    pub before: Option<usize>,

    /// Number of lines to search for context 
    #[arg(short = 'C', default_value_t = 0)]
    pub context: usize,

}
