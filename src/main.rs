mod callable;
mod env;
mod error;
mod expr;
mod interpreter;
mod lox_value;
mod native_fn;
mod parser;
mod rlox;
mod scanner;
mod stmt;
mod token;
mod visitor;

use anyhow::Result;
use clap::{App, Arg};
use rlox::{run_file, run_prompt};

use std::process::exit;

fn main() -> Result<()> {
    let matches = App::new("input")
        .arg(Arg::new("input").index(1))
        .get_matches();

    if let Some(i) = matches.value_of("input") {
        println!("{}", i);
        run_file(i.to_string())?;
    } else {
        run_prompt()?;
    }

    exit(0);
}
