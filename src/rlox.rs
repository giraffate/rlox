use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::io::stdin;

use crate::env::Env;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub fn run_file(path: String) -> Result<()> {
    let mut interpreter = Interpreter { env: Env::new() };
    let s =
        read_to_string(path.clone()).with_context(|| format!("couldn't read file `{}`", path))?;
    run(&mut interpreter, s);

    Ok(())
}

pub fn run_prompt() -> Result<()> {
    let mut interpreter = Interpreter { env: Env::new() };
    loop {
        let mut s = String::new();
        match stdin().read_line(&mut s).ok() {
            Some(_) => {}
            None => break,
        }
        run(&mut interpreter, s);
    }

    Ok(())
}

fn run(interpreter: &mut Interpreter, s: String) {
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
    match interpreter.interpret(stmts) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}
