use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use clap::Parser;

/// Struct to hold CLI arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The name of the file to read
    filename: Option<String>,
    /// Print line numbers
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    number: bool,
    /// Print $ at the end of each line
    #[arg(short='e', long, action = clap::ArgAction::SetTrue)]
    show_ends: bool,
}

/// This program takes a file name as an argument and prints the contents of the file.
fn main() {
    let cli: Cli = Cli::parse();
    let filename: String = cli.filename.unwrap_or_else(|| {
        eprintln!("Write filename to read.");
        std::process::exit(1)
    });
    let contents: File = File::open(filename).unwrap_or_else(|_| {
        eprintln!("An error occurred while opening the file.");
        std::process::exit(1)
    });
    let file_buffer: BufReader<File> = BufReader::new(contents);

    for (counter, line) in (1_u32..).zip(file_buffer.lines()) {
        let mut line: String = line.unwrap_or_else(|_| {
            eprintln!("An error occurred while reading the file.");
            std::process::exit(1);
        });
        if cli.number {
            line.insert_str(0, &format!("     {} ", counter));
        };
        if cli.show_ends {
            line.push('$');
        }
        println!("{}", line);
    }
}
