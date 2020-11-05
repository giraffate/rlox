use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::io::stdin;

use crate::env::Env;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub fn run_file(path: String) -> Result<()> {
    let s =
        read_to_string(path.clone()).with_context(|| format!("couldn't read file `{}`", path))?;
    run(s);

    Ok(())
}

pub fn run_prompt() -> Result<()> {
    loop {
        let mut s = String::new();
        match stdin().read_line(&mut s).ok() {
            Some(_) => {}
            None => break,
        }
        run(s);
    }

    Ok(())
}

fn run(s: String) {
    let mut scanner = Scanner {
        source: s.chars().collect(),
        ..Default::default()
    };
    let tokens = scanner.scan_tokens();
    let mut parser = Parser {
        tokens: tokens,
        current: 0,
    };
    let stmts = parser.parse();
    let mut interpreter = Interpreter { env: Env::new() };
    match interpreter.interpret(stmts) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}
