use std::fmt::Display;
use std::fs;
use anyhow::{anyhow, Error};
use clap::Parser;
use crate::digit::Digit;

mod digit;

#[derive(Parser, Debug)]
#[command()]
struct ClapArgs {
    #[arg(short, long)]
    replace: bool
}


fn main() -> Result<(), Error> {
    let args = ClapArgs::parse();

    println!("Day 1!");
    let input_sanitized = match fs::read_to_string("input.txt") {
        Ok(raw_contents) => {
            println!("Successfully read input!");

            let mut contents = raw_contents.to_lowercase();

            if args.replace {
                for digit in Digit::iterator() {
                    contents = contents.replace(
                        &digit.to_string(),
                        &digit.as_protected_num()
                    )
                }
            }

            println!("Converted input's number words to numbers!");

            contents
        },
        Err(err) => {
            return Err(anyhow!("Failed to read contents of input file: {}", err))
        }
    };

    let mut total = 0;


    for line in input_sanitized.split("\n") {
        if line.is_empty() {
            continue
        }

        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        // First digit in line
        for char in line.chars() {
            if char.to_digit(10).is_some() {
                first_digit = Some(char);
                break;
            }
        }

        // Last digit in line
        for char in line.chars().rev() {
            if char.to_digit(10).is_some() {
                last_digit = Some(char);
                break;
            }
        }

        let line_total = format!(
            "{}{}", first_digit.unwrap(), last_digit.unwrap()
        ).parse::<i32>()?;

        total += line_total;

        println!("{} => {}", line, line_total)
    }

    println!("Answer:\n\n{}\n\n:3", total);

    Ok(())
}
