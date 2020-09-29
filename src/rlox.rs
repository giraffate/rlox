use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::io::stdin;

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
    print!("{}", s);
}
