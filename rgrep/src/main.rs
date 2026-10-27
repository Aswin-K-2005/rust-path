use std::env;
use std::fs;

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
    let config = Config { pattern, path };

    let contents = match fs::read_to_string(&config.path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cant read the {}: {}  ", config.path, e);
            std::process::exit(1);
        }
    };
    let text = search(&config.pattern, &contents);
    for v in text {
        println!("{}", v);
    }
}

fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();
    for line in contents.lines() {
        if line.contains(pattern) {
            out.push(line);
        }
    }
    out
}
