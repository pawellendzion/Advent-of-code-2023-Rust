use std::{env, fs, result};

mod day1;

const COMPLETED_DAYS: u8 = 1;

type Result<T> = result::Result<T, String>;

fn main() {
    let path = env::args().next().unwrap();
    let name_of_bin = path.split('/').last().unwrap();

    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        eprintln!();
        eprintln!("usage: {} [DAY (1 - {})] [PUZZLE_DATA_PATH]", name_of_bin, COMPLETED_DAYS);
    }
}


fn run() -> Result<()> {
    let (day, puzzle_path) = get_args()?;

    let puzzle_data = fs::read_to_string(puzzle_path)
        .map_err(|_| "Cannot read file with puzzle data")?;

    let solution = solve(day, puzzle_data)?;

    println!("solution = {solution}");

    Ok(())
}

fn get_args() -> Result<(u8, String)> {
    let mut args = env::args();

    if args.len() < 3 {
        return Err(String::from("Not enough arguments"));
    }

    args.next();

    let day = args.next()
        .ok_or("Invalid argument [DAY]")?;

    let day: u8 = day.parse()
        .map_err(|_| "Argument [DAY] has to be a number")?;

    if day < 1 || day > COMPLETED_DAYS {
        return Err(format!("Invalid [DAY] number, available 1 - {}", COMPLETED_DAYS));
    }

    let puzzle_path = args.next()
        .ok_or("Invalid arguments [PUZZLE_DATA_PATH]")?;

    Ok((day, puzzle_path))
}

fn solve(day: u8, puzzle_data: String) -> Result<String> {
    let result: String = match day {
        1 => day1::slove(puzzle_data).to_string(),
        _ => return Err(format!("Not found solution for day {}", day)),
    };

    Ok(result)
}
