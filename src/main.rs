use clap::Parser;
use std::{fs::metadata, path::PathBuf};

#[derive(Parser)]
struct Cli {
    file: PathBuf,
    #[arg(short = 'c')]
    #[arg(long = "bytes")]
    bytes: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.bytes {
        let bytes = count_bytes(&cli.file).unwrap();
        let output_path = &cli.file.to_str().unwrap();
        println!("{} {}", bytes, output_path)
    }
}

fn count_bytes(path: &PathBuf) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(metadata(path)?.len())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::count_bytes;

    #[test]
    fn check_bytes() {
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("test_files/test.txt");
        assert_eq!(342190, count_bytes(&test_file).unwrap());
    }
}
