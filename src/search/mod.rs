pub mod literal;
pub mod ignore_case;
pub mod regex_search;
pub mod fuzzy;

use crate::cli::Args;
use crate::output::{print_match, SearchResult};
use crate::walker::collect_files;

use regex::Regex;

use std::{fs, result};
use std::io;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub enum SearchMode {
    Literal(String),
    IgnoreCase(String),
    Regex(Regex),
    Fuzzy(String),
}

impl SearchMode {
    pub fn matches(&self, line: &str) -> bool{
        match self {
            SearchMode::Literal(query) => {
                literal::matches(line, query)
            }

            SearchMode::IgnoreCase(query) => {
                ignore_case::matches(line, query)
            }

            SearchMode::Regex(query) => { 
                regex_search::matches(line, query)
            }

            SearchMode::Fuzzy(query) => {
                fuzzy::matches(line, query)
            }
        }
    }
}

// Add fuzzy finding later
fn build_search_mode(args: &Args) -> SearchMode {
    if args.enable_regex {
        SearchMode::Regex(
            Regex::new(&args.query)
                .expect("Invalid regex pattern"),
        )
    } else if args.ignore_case {
        SearchMode::IgnoreCase(
            args.query.to_lowercase(),
        )
    } else {
        SearchMode::Literal(
            args.query.clone(),
        )
    }
}

fn build_context(lines: &[&str], match_index: usize, context: usize,) -> Vec<(usize, String)>{

    // Prevents going out of bounds on LHS
    let start = match_index.saturating_sub(context);

    // Prevents going out of bounds on RHS
    let end = (match_index + context + 1).min(lines.len());

    let mut result = Vec::new();

    for i in start..end {
        result.push((i + 1, lines[i].to_string(),
    ));
    }
    result
}


fn spawn_workers(
    worker_count: usize,
    mode: Arc<SearchMode>,
    path_receiver: Arc<Mutex<mpsc::Receiver<PathBuf>>>,
    result_sender: mpsc::Sender<SearchResult>,
    context: usize,)
    {

    for _ in 0..worker_count{

        let mode = Arc::clone(&mode);
        let path_receiver = Arc::clone(&path_receiver);
        let result_sender = result_sender.clone();

        thread::spawn(move || {

            loop {

                let path = {
                    let receiver = path_receiver.lock().unwrap();
                    receiver.recv()
                };

                let path = match path {
                    Ok(path) => path,
                    Err(_) => break,
                };

                let contents = match fs::read_to_string(&path) {
                    Ok(text) => text,
                    Err(_) => continue,
                };

                let lines: Vec<&str> = contents.lines().collect();

                for(line_number, line) in lines.iter().enumerate() {
                if mode.matches(line){
                   // Store our data in result

                    let result = SearchResult{
                        path: path.display().to_string(),

                        match_line: line_number + 1,

                        context: build_context(&lines, line_number, context),
                    };

                    let _ = result_sender.send(result);

                    }
                }
            }
        });
    }
}


pub fn run_search(args:&Args) -> io::Result<()> {

    // Build search mode
    let mode = Arc::new(build_search_mode(args));

    // Create channels
    let (path_sender, path_receiver) = 
        mpsc::channel::<PathBuf>();
    
    let path_receiver = Arc::new(Mutex::new(path_receiver));

    let (result_sender, result_receiver) =
        mpsc::channel::<SearchResult>();

    // Create workers

    spawn_workers(4, Arc::clone(&mode), Arc::clone(&path_receiver), result_sender, args.context);

    // Send files
    let files = collect_files(&args.path);

    for file in files {
        let _ = path_sender.send(file);
    }

    drop(path_sender);

    // Printing results
    for result in result_receiver{
        print_match(&result);
    }

    Ok(())
}