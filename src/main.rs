#!/usr/bin/env rust-script

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct CatArgs {
    show_lines: bool,
    show_help: bool,
    file_paths: Vec<String>,
}

fn parse_args() -> CatArgs {
    let argv = env::args().into_iter().skip(1);

    let mut show_lines = false;
    let mut show_help = false;

    let mut file_paths = Vec::new();

    for arg in argv {
        if arg == "--n" {
            show_lines = true
        } else if arg == "--h" {
            show_help = true
        } else {
            file_paths.push(arg);
        }
    }

    CatArgs {
        show_lines,
        show_help,
        file_paths,
    }
}

fn show_help_and_exit() {
    println!(
        "\
cat-rs — A minimal clone of the Unix `cat` command

USAGE:
    cat-rs [OPTIONS] <FILES>...

OPTIONS:
    --n         Show line numbers
    --h         Show this help message

EXAMPLES:
    cat-rs file.txt
    cat-rs --n file1.txt file2.txt
"
    );
    std::process::exit(0);
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let cat_args = parse_args();

    if cat_args.show_help {
        show_help_and_exit();
    }

    if cat_args.file_paths.is_empty() {
        let reader = BufReader::new(std::io::stdin());
        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            if cat_args.show_lines {
                println!("{} {}", line_num + 1, line);
            } else {
                println!("{}", line);
            }
        }
    } else {
        for filepath in cat_args.file_paths.iter() {
            let file = File::open(filepath)?;
            let reader = BufReader::new(file);
            for (line_num, line_result) in reader.lines().enumerate() {
                let line = line_result?;
                if cat_args.show_lines {
                    println!("{} {}", line_num + 1, line);
                } else {
                    println!("{}", line);
                }
            }
        }
    }

    Ok(())
}
