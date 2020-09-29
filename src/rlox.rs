use anyhow::Result;
use std::io::stdin;
use std::fs::read_to_string;

pub fn run_file(path: String) -> Result<()> {
    let s = read_to_string(path)?;
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
