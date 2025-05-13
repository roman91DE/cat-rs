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
    let argv = env::args().skip(1);

    let mut show_lines = false;
    let mut show_help = false;
    let mut file_paths = Vec::new();

    for arg in argv {
        if arg == "-n" || arg == "--line_numbers" {
            show_lines = true;
        } else if arg == "-h" || arg == "--help" {
            show_help = true;
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
    cat-rs [OPTIONS] [FILES]...

OPTIONS:
    -n, --line_numbers     Show line numbers in the output
    -h, --help             Show this help message and exit

DESCRIPTION:
    Reads files sequentially, writing them to standard output.
    If no files are specified, or a file is '-', reads from standard input.

EXAMPLES:
    cat-rs file.txt
    cat-rs -n file1.txt file2.txt
    echo \"hello\" | cat-rs -n
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
        for (idx, line_result) in reader.lines().enumerate() {
            let line_num = idx +1;
            let line = line_result?;
            if cat_args.show_lines {
                println!("{line_num} {line}");
            } else {
                println!("{line}");
            }
        }
    } else {
        for filepath in &cat_args.file_paths {
            let file = File::open(filepath)?;
            let reader = BufReader::new(file);
            for (line_num, line_result) in reader.lines().enumerate() {
                let line = line_result?;
                if cat_args.show_lines {
                    println!("{line_num} {line}");
                } else {
                    println!("{line}");
                }
            }
        }
    }

    Ok(())
}
