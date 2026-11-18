use std::env;
use std::fs;

struct Config {
    pattern: String,
    path: String,
    ignore_case: bool,
}

fn main() {
    let mut ignore_case = false;
    let mut rest = Vec::new();
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-i" => ignore_case = true,
            _ => rest.push(arg),
        }
    }

    let pattern = match rest.get(0) {
        Some(p) => p.clone(),
        None => {
            eprintln!("Pattern missing T-T");
            std::process::exit(1);
        }
    };
    let path = match rest.get(1) {
        Some(n) => n.clone(),
        None => {
            eprintln!("Path missing T-T");
            std::process::exit(1);
        }
    };
    let config = Config {
        pattern,
        path,
        ignore_case,
    };

    let contents = match fs::read_to_string(&config.path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cant read the {}: {}  ", config.path, e);
            std::process::exit(1);
        }
    };
    let text = search(&config.pattern, &contents, config.ignore_case);
    for v in text {
        println!("{}", v);
    }
}

fn search<'a>(pattern: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut out = Vec::new();
    for line in contents.lines() {
        let matched = if ignore_case {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };
        if matched {
            out.push(line);
        }
    }
    out
}
