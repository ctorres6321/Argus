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

    /// Enables context for pattern matching (Prints N number of lines below and above the matched term)
    #[arg(short = 'c', long)]
    pub enable_context: bool, 

    // Enables context for pattern matching (Prints N number of lines after the pattern was found)
//  #[arg(short = 'a', long)]
//  pub enable_context_after: bool, 

    // Enables context for pattern matching (Prints N number of lines before the pattern was found)
//  #[arg(short = 'b', long)]
//  pub enable_context_before: bool, 

}

impl Args {
    pub fn _get_query(self) -> String{
      self.query
    }

    pub fn _get_path(self) -> String{
        self.path
    }
}