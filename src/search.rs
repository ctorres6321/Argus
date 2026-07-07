use crate::cli::Args;
use core::num;
use std::fs;
use std::io;
use std::path;
use std::sync::mpsc::Receiver;
use walkdir::WalkDir;
use std::thread;
use regex::Regex;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};


pub fn run_search(args: &Args) -> io::Result<()> {

    //Create the two cases where we have ignore case or don't have ignore case

    let query = if args.ignore_case{
        args.query.to_lowercase()
    }
    else{
        args.query.clone()
    };

    let query = Arc::new(query);
    let ignore_case = args.ignore_case;




    

    // Create the sender handle and the unique receiver channel for the threads
    let(path_sender, path_receiver) = mpsc::channel::<PathBuf>();
    let path_receiver = Arc::new(Mutex::new(path_receiver));

    let (result_sender, result_receiver) = mpsc::channel::<String>();

    // Number of threads to search thru directories
    let worker_count = 4;


    //Logic for concurrency 
    for _ in 0..worker_count{
        let path_receiver = Arc::clone(&path_receiver);
        let result_sender = result_sender.clone();
        let query = Arc::clone(&query);

        thread::spawn(move|| {
            loop {
                let path = {
                    let receiver = path_receiver.lock().unwrap();
                    receiver.recv()
                };

                let path = match path {
                    Ok(path) => path,
                    Err(threads) => break,
                };

                let contents = match fs::read_to_string(&path){
                    Ok(text) => text,
                    Err(_) => continue,
                };

                for(line_number, line) in contents.lines().enumerate(){
                    let line_to_check = if ignore_case{
                        line.to_lowercase()
                    }
                    else {
                        line.to_string()
                    };

                    if line_to_check.contains(query.as_str()){
                        let result = format!(
                            "{}:{}: {}", path.display(), line_number + 1, line
                        );

                        let _ = result_sender.send(result);

                    }

                }


            }
        });
        
    }

    drop(result_sender);

    for entry in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()){
        let path = entry.path();

        if path.components().any(|c| {
            let s = c.as_os_str();
            s == ".git" || s == "target"
        }){
            continue;
        }

        if path.is_file(){
            let _ = path_sender.send(path.to_path_buf());
        }
    }

    drop(path_sender);
    
    for result in result_receiver{
        println!("{result}");
    }


    Ok(())
}