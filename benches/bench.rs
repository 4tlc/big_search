#![allow(unused)] // fix unused warnings printing on *cargo test*

use std::process::Command;
use std::time::Instant;
#[path = "../src/main.rs"]
mod main;
use crate::main::MATCHED_FILES;
use crate::main::SEARCHED_SIZE;
use crate::main::TOTAL_SIZE;
#[path = "../src/errors.rs"]
mod errors;
#[path = "../src/search_files.rs"]
mod search_files;
use crate::search_files::loop_files;
#[path = "../src/parse_args.rs"]
mod parse_args;
use crate::parse_args::parse_args;

fn main() {
    // to print the number of files matched grep -l -r this .doc | wc -l
    compare_grep(
        "benches/to_search/personal_website_node_modules",
        "Personal Website Node Modules",
    );
    compare_grep(
        "benches/to_search/sorting_visualizer_node_modules",
        "Sorting Visualizer Node Modules",
    );
}

// bench against grep on large node modules
fn compare_grep(folder: &str, name: &str) {
    println!("\x1b[04\x1b[47mGrep Comparison on {}\x1b[0m", name);
    let time = Instant::now();
    let (path, target) = parse_args(vec!["_".to_string(), folder.to_string(), "a".to_string()]);

    loop_files(&target, std::fs::read_dir(path).unwrap());
    println!("Time for bs: {:?}", time.elapsed());
    let time = Instant::now();
    let mut grep_cmd = Command::new("grep")
        .args(["-l", "-r", "a", folder])
        .output()
        .expect("grep")
        .stdout;
    // i think these are printing out line numbers of located files
    println!("Grep Time {:?}", time.elapsed());
}
