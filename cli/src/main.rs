use std::{env, error::Error, path::PathBuf, str::FromStr};

use solutions::DAYS;

struct Args {
    day_number: u32,
    part_number: u32,
    input_filename: PathBuf,
}

fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();
    Args {
        day_number: args[1].parse().unwrap(),
        part_number: args[2].parse().unwrap(),
        input_filename: PathBuf::from_str(&args[3]).unwrap(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    if let Some(day) = DAYS.get((args.day_number - 1) as usize) {
        if let Some(solution) = &day.solution {
            let input = std::fs::read_to_string(args.input_filename)?;
            let answer = match args.part_number {
                1 => (solution.solve_part_1)(&input)?,
                2 => (solution.solve_part_2)(&input)?,
                _ => "???".to_string(),
            };
            println!("{}", answer);
        } else {
            println!("No solution for day {} yet.", args.day_number);
        }
    } else {
        println!("Invalid day number: {}.", args.day_number);
    }
    Ok(())
}
