use clap::Parser;
use std::{
    fs::{metadata, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    file: PathBuf,

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

    // TODO: Read the file in once, use the reference in all relevant functions

    // wc always uses this order: lines, words, chars, bytes, max-line-length

    // If no parameters have been provided, set the default -c -l -w
    if !(cli.lines || cli.words || cli.chars || cli.bytes) {
        cli.bytes = true;
        cli.lines = true;
        cli.words = true;
    }

    if cli.lines {
        let lines = count_lines(&cli.file).unwrap();

        output.push_str(lines.to_string().as_str());
        output.push(' ');
    }

    if cli.words {
        let words = count_words(&cli.file).unwrap();

        output.push_str(words.to_string().as_str());
        output.push(' ');
    }

    if cli.chars {
        let chars = count_chars(&cli.file).unwrap();

        output.push_str(chars.to_string().as_str());
        output.push(' ');
    }

    if cli.bytes {
        let bytes = count_bytes(&cli.file).unwrap();

        output.push_str(bytes.to_string().as_str());
        output.push(' ');
    }

    output.push_str(&cli.file.to_str().unwrap());
    println!("{}", output)
}

fn count_bytes(path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(metadata(path)?.len())
}

fn count_lines(path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    let num_lines = u64::try_from(lines.count()).unwrap();

    Ok(num_lines)
}

fn count_words(path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let mut num_words: u64 = 0;

    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let _ = lines
        .inspect(|line| {
            let _ = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|_| num_words += 1)
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    Ok(num_words)
}

fn count_chars(path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    let mut num_chars: u64 = 0;

    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let _ = lines
        .inspect(|line| {
            let _ = line
                .as_ref()
                .unwrap()
                .split("")
                .map(|_| num_chars += 1)
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    Ok(num_chars)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::*;

    fn get_test_file() -> PathBuf {
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("test_files/test.txt");

        test_file
    }

    #[test]
    fn check_bytes() {
        let test_file = get_test_file();
        assert_eq!(342190, count_bytes(&test_file).unwrap());
    }

    #[test]
    fn check_lines() {
        let test_file = get_test_file();
        assert_eq!(7145, count_lines(&test_file).unwrap());
    }

    #[test]
    fn check_words() {
        let test_file = get_test_file();
        assert_eq!(58164, count_words(&test_file).unwrap());
    }

    #[test]
    fn check_chars() {
        let test_file = get_test_file();
        assert_eq!(339292, count_chars(&test_file).unwrap());
    }
}
