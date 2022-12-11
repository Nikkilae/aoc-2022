use std::error::Error;

fn parse_command(line: &str) -> Option<i32> {
    let mut parts = line.split(' ');
    if parts.next()? == "addx" {
        return parts.next()?.parse::<i32>().ok();
    }
    None
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut signal_sum = 0;
    let mut cycle_number = 1;
    let mut x = 1;
    let mut cycle = |x| {
        match cycle_number {
            20 | 60 | 100 | 140 | 180 | 220 => signal_sum += cycle_number * x,
            _ => {}
        }
        cycle_number += 1;
    };
    for line in input.lines() {
        if let Some(n) = parse_command(line) {
            cycle(x);
            cycle(x);
            x += n;
        } else {
            cycle(x);
        }
    }
    Ok(signal_sum.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();
    let mut cycle_number = 1;
    let mut x = 1;
    let mut cycle = |x: i32| {
        let is_visible = (x - ((cycle_number - 1) % 40)).abs() <= 1;
        if is_visible {
            output += "#";
        } else {
            output += ".";
        }
        if cycle_number % 40 == 0 {
            output += "\n";
        }
        cycle_number += 1;
    };
    for line in input.lines() {
        if let Some(n) = parse_command(line) {
            cycle(x);
            cycle(x);
            x += n;
        } else {
            cycle(x);
        }
    }
    Ok(output)
}
