use std::fs::File;

use clap::Parser;
use employee_db::file::{create_db_file, open_db_file};

#[derive(Parser, Debug)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    /// File path
    #[arg(short = 'f')]
    file_path: Option<String>,

    /// new file
    #[arg(short = 'n')]
    new_file: bool,

    /// custom help flag
    #[arg(short = 'h', long = "help", action = clap::ArgAction::SetTrue)]
    help: bool,
}

fn print_help() {
    println!("Usage: cargo run -n -f <dbfilename>");
    println!("\t-n  Create a new db file");
    println!("\t-f  (required) Path to dbfilename");
}

fn main() {
    let args = Args::parse();

    if args.help {
        print_help();
        std::process::exit(0);
    }

    if args.file_path.is_none() {
        println!("File path cannot be empty\nUse -h to know usage");
        std::process::exit(1);
    }

    let file = if args.new_file {
        match create_db_file(&args.file_path.unwrap()) {
            Ok(file) => file,
            Err(e) => panic!("{e}"),
        }
    } else {
        match open_db_file(&args.file_path.unwrap()) {
            Ok(file) => file,
            Err(e) => panic!("{e}"),
        }
    };
}
