use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";

struct Config {
    pattern: String,
    path: String,
    ignore_case: bool,
    line_number: bool,
    invert: bool,
    recursive: bool,
}

enum RgrepError {
    Io(io::Error),
}

impl fmt::Display for RgrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RgrepError::Io(e) => write!(f, "io error: {}", e),
        }
    }
}
impl From<io::Error> for RgrepError {
    fn from(e: io::Error) -> Self {
        RgrepError::Io(e)
    }
}

struct Match<'a> {
    line_no: usize,
    text: &'a str,
}

fn highlight(line: &str, pattern: &str, ignore_case: bool) -> String {
    let found = if ignore_case {
        line.to_lowercase().find(&pattern.to_lowercase())
    } else {
        line.find(pattern)
    };
    match found {
        Some(t) => {
            let end = pattern.len() + t;
            let before = &line[..t];
            let matched = &line[t..end];
            let after = &line[end..];
            format!("{}{}{}{}{}", before, RED, matched, RESET, after)
        }
        None => line.to_string(),
    }
}

fn grep_file(config: &Config, path: &Path) -> Result<(), RgrepError> {
    if path.is_dir() {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let new_path = entry.path();
            if new_path.ends_with("target") {
                continue;
            }

            if let Err(e) = grep_file(config, &new_path) {
                eprintln!("rgrep: {}", e);
            }
        }
        return Ok(());
    }
    let contents = fs::read_to_string(path)?;
    let text = search(
        &config.pattern,
        &contents,
        config.ignore_case,
        config.invert,
    );
    let prefix = if config.recursive {
        format!("{} :", path.display())
    } else {
        String::new()
    };

    for v in text {
        if config.line_number {
            println!(
                "{}{} : {}",
                prefix,
                v.line_no,
                highlight(v.text, &config.pattern, config.ignore_case)
            );
        } else {
            println!(
                "{}{}",
                prefix,
                highlight(v.text, &config.pattern, config.ignore_case)
            );
        }
    }
    Ok(())
}

fn main() {
    let mut ignore_case = false;

    let mut line_number = false;
    let mut invert = false;
    let mut recursive = false;
    let mut rest = Vec::new();
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-i" => ignore_case = true,
            "-n" => line_number = true,
            "-v" => invert = true,
            "-r" => recursive = true,
            _ => rest.push(arg),
        }
    }

    let pattern = match rest.first() {
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
        recursive,
    };
    let new_path = Path::new(&config.path);

    if new_path.is_dir() && !config.recursive {
        eprintln!("{} is a folder use -r ", new_path.display());
        std::process::exit(1);
    } else {
        if let Err(e) = grep_file(&config, new_path) {
            eprintln!("rgrep: {}", e);
            std::process::exit(1);
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
