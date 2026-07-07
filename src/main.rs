mod cli;
mod search;
use clap::Parser;
use cli::Args;

fn main() {

     let args = Args::parse();

     if let Err(err) = search::run_search(&args) {
         eprintln!("Error: {err}");
     }

}