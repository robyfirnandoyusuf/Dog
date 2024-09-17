use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const PAGE_SIZE: usize = 20;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: dog <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut line_number = 0;
    let mut current_page = 0;
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(format!("{: <4} {}", line_number + 1, line));
        line_number += 1;

        if line_number % PAGE_SIZE == 0 {
            if should_display_page(current_page) {
                display_page(&lines, current_page)?;
                lines.clear();
                current_page += 1;
            }
        }
    }

    if !lines.is_empty() && should_display_page(current_page) {
        display_page(&lines, current_page)?;
    }

    Ok(())
}

fn should_display_page(page_number: usize) -> bool {
    let mut input = String::new();

    println!("Press Enter to show next page or 'q' to quit.");

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim() == "q" {
                false
            } else {
                true
            }
        }
        Err(_) => false,
    }
}

fn display_page(lines: &Vec<String>, page_number: usize) -> io::Result<()> {
    println!("Page {}:\n", page_number + 1);

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
