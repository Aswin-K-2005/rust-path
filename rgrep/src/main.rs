use std::{env, os};

struct Config {
    pattern: String,
    path: String,
}

fn main() {
    let mut argument = env::args();
    argument.next();

    let pattern = match argument.next() {
        Some(p) => p,
        None => {
            eprintln!("Pattern missing T-T");
            std::process::exit(1);
        }
    };
    let path = match argument.next() {
        Some(n) => n,
        None => {
            eprintln!("Path missing T-T");
            std::process::exit(1);
        }
    };
    let new = Config { pattern, path };
    println!(" pattern: {}  path: {}", new.pattern, new.path);
}
