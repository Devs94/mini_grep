#![allow(unused)]

use std::fmt::format;

use clap::Parser;
use regex::{Regex, Captures};
use colored::*;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    let file_content = std::fs::read_to_string(&args.path).expect("Erreur de lecture...");
    let fc_8 = file_content.as_str();
    let re = Regex::new(&args.pattern).unwrap();
    let final_repr = re.replace_all(fc_8, |caps: &Captures| {
        format!("{}", &caps[0].bright_red())
    });
    println!("{}", final_repr)
}