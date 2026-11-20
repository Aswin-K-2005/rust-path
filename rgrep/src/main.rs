use std::env;
use std::fs;

struct Config {
    pattern: String,
    path: String,
    ignore_case: bool,
    line_number: bool,
    invert: bool,
}

struct Match<'a> {
    line_no: usize,
    text: &'a str,
}

fn main() {
    let mut ignore_case = false;

    let mut line_number = false;
    let mut invert = false;
    let mut rest = Vec::new();
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-i" => ignore_case = true,
            "-n" => line_number = true,
            "-v" => invert = true,
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
        line_number,
        invert,
    };

    let contents = match fs::read_to_string(&config.path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cant read the {}: {}  ", config.path, e);
            std::process::exit(1);
        }
    };
    let text = search(
        &config.pattern,
        &contents,
        config.ignore_case,
        config.invert,
    );
    for v in text {
        if config.line_number {
            println!("{} : {}", v.line_no, v.text);
        } else {
            println!("{}", v.text);
        }
    }
}

fn search<'a>(pattern: &str, contents: &'a str, ignore_case: bool, invert: bool) -> Vec<Match<'a>> {
    let mut out = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        let mut matched = if ignore_case {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };
        if invert {
            matched = !matched;
        }
        if matched {
            out.push(Match {
                line_no: i + 1,
                text: line,
            });
        }
    }
    out
}
