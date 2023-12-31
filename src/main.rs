use std::{env, fs, result};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const COMPLETED_DAYS: u8 = 9;

type Result<T> = result::Result<T, String>;

pub enum Part { First, Second }

fn main() {
    let path = env::args().next().unwrap();
    let name_of_bin = path.split('/').last().unwrap();

    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        eprintln!();
        eprintln!("usage: {} [DAY (1 - {})] [PART (1 or 2)] [PUZZLE_DATA_PATH]", name_of_bin, COMPLETED_DAYS);
    }
}


fn run() -> Result<()> {
    let (day, part, puzzle_path) = get_args()?;

    let puzzle_data = fs::read_to_string(puzzle_path)
        .map_err(|_| "Cannot read file with puzzle data")?;

    let solution = solve(day, part, puzzle_data)?;

    println!("solution = {solution}");

    Ok(())
}

fn get_args() -> Result<(u8, Part, String)> {
    let mut args = env::args();

    if args.len() < 4 {
        return Err(String::from("Not enough arguments"));
    }

    args.next();

    let day: u8 = args.next()
        .ok_or("Invalid argument [DAY]")?
        .parse().map_err(|_| "Argument [DAY] has to be a number")?;

    if day < 1 || day > COMPLETED_DAYS {
        return Err(format!("Invalid [DAY] number, available 1 - {}", COMPLETED_DAYS));
    }

    let part: u8 = args
        .next().ok_or("Invalid argument [PART]")?
        .parse().map_err(|_| "Invalid argument [PART]")?;
    
    let part = match part {
        1 => Part::First,
        2 => Part::Second,
        _ => return Err(String::from("Invalid [PART] number, available 1 - 2")), 
    };

    let puzzle_path = args.next()
        .ok_or("Invalid arguments [PUZZLE_DATA_PATH]")?;

    Ok((day, part, puzzle_path))
}

fn solve(day: u8, part: Part, puzzle_data: String) -> Result<String> {
    let result = match day {
        1 => day1::Solution::new(part).solve(puzzle_data).to_string(),
        2 => day2::Solution::new(part).solve(puzzle_data).to_string(),
        3 => day3::Solution::new(part).solve(puzzle_data).to_string(),
        4 => day4::Solution::new(part).solve(puzzle_data).to_string(),
        5 => day5::Solution::new(part).solve(puzzle_data).to_string(),
        6 => day6::Solution::new(part).solve(puzzle_data).to_string(),
        7 => day7::Solution::new(part).solve(puzzle_data).to_string(),
        8 => {
            let answers = day8::solve(puzzle_data).unwrap();
            format!("part_1 = {}\npart_2 = {}", answers.0, answers.1)
        },
        9 => {
            let answers = day9::solve(puzzle_data).unwrap();
            format!("part_1 = {}\npart_2 = {}", answers.0, answers.1)
        }
        _ => return Err(format!("Not found solution for day {}", day)),
    };

    Ok(result)
}

