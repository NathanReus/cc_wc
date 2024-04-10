use clap::Parser;
use std::{
    fs::{self, metadata, File},
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    file: Option<PathBuf>,

    #[arg(short = 'c')]
    #[arg(long = "bytes")]
    bytes: bool,

    #[arg(short = 'l')]
    #[arg(long = "lines")]
    lines: bool,

    #[arg(short = 'w')]
    #[arg(long = "words")]
    words: bool,

    #[arg(short = 'm')]
    #[arg(long = "chars")]
    chars: bool,
}

fn main() {
    let mut cli = Cli::parse();
    let mut output = String::new();
    let mut using_stdin = false;
    let mut num_lines: u64 = 0;

    // Check whether a filename was provided. Open it or read from stdin
    let contents = match &cli.file {
        Some(file) => fs::read_to_string(file).unwrap(),
        None => {
            using_stdin = true;

            let stdin = stdin();
            let mut buffer = String::new();
            loop {
                match stdin.read_line(&mut buffer) {
                    Ok(len) => {
                        if len == 0 {
                            break;
                        } else {
                            num_lines += 1;
                        }
                    }
                    Err(_error) => {
                        panic!("Failed to read file");
                    }
                }
            }

            buffer
        }
    };

    // If no parameters have been provided, set the default -c -l -w
    if !(cli.lines || cli.words || cli.chars || cli.bytes) {
        cli.bytes = true;
        cli.lines = true;
        cli.words = true;
    }

    // wc always uses this order: lines, words, chars, bytes, max-line-length
    if cli.lines {
        let mut lines = 0;
        if using_stdin {
            lines = num_lines;
        } else {
            lines = count_lines(&contents).unwrap();
        }

        output.push_str(lines.to_string().as_str());
        output.push(' ');
    }

    if cli.words {
        let words = count_words(&contents).unwrap();

        output.push_str(words.to_string().as_str());
        output.push(' ');
    }

    if cli.chars {
        let chars = count_chars(&contents).unwrap();

        output.push_str(chars.to_string().as_str());
        output.push(' ');
    }

    if cli.bytes {
        let bytes = count_bytes(&contents).unwrap();

        output.push_str(bytes.to_string().as_str());
        output.push(' ');
    }

    if !using_stdin {
        output.push_str(&cli.file.unwrap().to_str().unwrap());
    }

    println!("{}", output)
}

fn count_bytes(contents: &String) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(u64::try_from(contents.len())?)
}

fn count_lines(contents: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = contents.lines();
    let num_lines = u64::try_from(lines.count()).unwrap();

    Ok(num_lines)
}

fn count_words(contents: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut num_words: u64 = 0;

    let lines = contents.lines();

    let _ = lines
        .inspect(|line| {
            let _ = line
                .split_whitespace()
                .map(|_| num_words += 1)
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    Ok(num_words)
}

fn count_chars(contents: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut num_chars: u64 = 0;

    let lines = contents.lines();

    let _ = lines
        .inspect(|line| {
            let _ = line.split("").map(|_| num_chars += 1).collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    Ok(num_chars)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::*;

    fn read_test_file() -> String {
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("test_files/test.txt");

        fs::read_to_string(test_file).unwrap()
    }

    #[test]
    fn check_bytes() {
        let test_file = read_test_file();
        assert_eq!(342190, count_bytes(&test_file).unwrap());
    }

    #[test]
    fn check_lines() {
        let test_file = read_test_file();
        assert_eq!(7145, count_lines(&test_file).unwrap());
    }

    #[test]
    fn check_words() {
        let test_file = read_test_file();
        assert_eq!(58164, count_words(&test_file).unwrap());
    }

    #[test]
    fn check_chars() {
        let test_file = read_test_file();
        assert_eq!(339292, count_chars(&test_file).unwrap());
    }
}
