use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

struct CatOptions {
    show_number: bool,
    number_nonblank: bool,
    show_ends: bool,
    show_tabs: bool,
}

fn print_reader<R: BufRead>(reader: R, opts: &CatOptions, line_counter: &mut usize) -> io::Result<()> {
    for line in reader.lines() {
        let mut line = line?;
        if opts.show_tabs {
            line = line.replace('\t', "^I");
        }

        let mut prefix = String::new();
        let is_blank = line.is_empty();
        if opts.show_number {
            if opts.number_nonblank {
                if !is_blank {
                    prefix = format!("{:6}\t", *line_counter);
                    *line_counter += 1;
                }
            } else {
                prefix = format!("{:6}\t", *line_counter);
                *line_counter += 1;
            }
        }

        if opts.show_ends {
            line.push('$');
        }

        println!("{}{}", prefix, line);
    }
    Ok(())
}

fn process_file(path: &str, opts: &CatOptions, line_counter: &mut usize) -> io::Result<()> {
    if path == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        print_reader(reader, opts, line_counter)
    } else {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        print_reader(reader, opts, line_counter)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut opts = CatOptions {
        show_number: false,
        number_nonblank: false,
        show_ends: false,
        show_tabs: false,
    };
    let mut files: Vec<String> = Vec::new();
    let mut parsing_flags = true;

    for arg in &args[1..] {
        if parsing_flags && arg == "--" {
            parsing_flags = false;
            continue;
        }

        if parsing_flags && arg.starts_with('-') && arg != "-" {
            for ch in arg.chars().skip(1) {
                match ch {
                    'n' => opts.show_number = true,
                    'b' => {
                        opts.number_nonblank = true;
                        opts.show_number = true;
                    }
                    'E' => opts.show_ends = true,
                    'T' => opts.show_tabs = true,
                    _ => {
                        eprintln!("rcat: invalid option -- {}", ch);
                        eprintln!("Usage: rcat [-b] [-E] [-n] [-T] [file ...]");
                        std::process::exit(1);
                    }
                }
            }
        } else {
            parsing_flags = false;
            files.push(arg.clone());
        }
    }

    if files.is_empty() {
        files.push("-".to_string());
    }

    let mut line_counter = 1;

    for file in files {
        if let Err(e) = process_file(&file, &opts, &mut line_counter) {
            eprintln!("rcat: {}: {}", file, e);
        }
    }

    io::stdout().flush()?;
    Ok(())
}
