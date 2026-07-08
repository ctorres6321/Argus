use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {

    /// Text to search for
    pub query: String,

    /// Directory to search
    pub path: String,

    /// Ignore case distinctions
    #[arg(short = 'c', long)]
    pub ignore_case: bool, 

    /// Enable regex support
    #[arg(short = 'r', long)]
    pub enable_regex: bool, 

}

impl Args {
    pub fn _get_query(self) -> String{
      self.query
    }

    pub fn _get_path(self) -> String{
        self.path
    }
}