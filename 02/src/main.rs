use std::fs;
use anyhow::Error;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;

    let mut game_id_total: i32 = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            continue
        }

        let mut is_game_valid = true;

        let game_id = str::parse::<i8>(
            &*get_between(line, "Game ", ":")
        )?;

        let game = line.trim_start_matches(&format!("Game {game_id}: "));

        let game_content = game.split(";");

        const RED_LIMIT: i32 = 12;
        const GREEN_LIMIT: i32 = 13;
        const BLUE_LIMIT: i32 = 14;

        for set in game_content {
            let mut red_total = 0;
            let mut green_total = 0;
            let mut blue_total = 0;

            for pull in set.split(", ") {
                if pull.contains("red") {
                    let pull_amount = pull
                        .split_whitespace()
                        .collect::<Vec<&str>>()[0]
                        .parse::<i32>()?;
                    red_total += pull_amount;
                } else if pull.contains("green") {
                    let pull_amount = pull
                        .split_whitespace()
                        .collect::<Vec<&str>>()[0]
                        .parse::<i32>()?;
                    green_total += pull_amount;
                } else if pull.contains("blue") {
                    let pull_amount = pull
                        .split_whitespace()
                        .collect::<Vec<&str>>()[0]
                        .parse::<i32>()?;
                    blue_total += pull_amount;
                }
            }

            if red_total > RED_LIMIT || green_total > GREEN_LIMIT || blue_total > BLUE_LIMIT {
                is_game_valid = false;
                break;
            }
        }

        if is_game_valid {
            game_id_total += i32::from(game_id)
        } else {
            continue
        }
    }

    println!("Game ID Total: {game_id_total}");

    Ok(())
}

fn get_between(input: &str, start: &str, end: &str) -> String {
    let mut output = input;

    output = output.trim_start_matches(start);

    // snippet taken from a stackoverflow answer:
    // https://stackoverflow.com/a/72362644/15076207
    output = &output[0..output.find(end).unwrap_or(output.len())];

    output.to_string()
}
