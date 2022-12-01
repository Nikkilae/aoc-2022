use std::error::Error;

use aoc_2022::read_lines;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Part 1: {}",
        solve_part_1(read_lines("input/d1_1_real.txt")?)?
    );
    println!(
        "Part 2: {}",
        solve_part_2(read_lines("input/d1_2_real.txt")?)?
    );
    Ok(())
}

fn solve_part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> Result<i32, Box<dyn Error>> {
    let mut max_total = 0;
    let mut current_total = 0;
    for line in lines {
        let line = line.as_ref();
        if line.is_empty() {
            if current_total > max_total {
                max_total = current_total;
            }
            current_total = 0;
        } else {
            current_total += str::parse::<i32>(line)?;
        }
    }
    Ok(max_total)
}

fn solve_part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> Result<i32, Box<dyn Error>> {
    let mut top_3_totals = Vec::new();
    let mut current_total = 0;
    for line in lines {
        let line = line.as_ref();
        if line.is_empty() {
            if top_3_totals.len() < 3 {
                top_3_totals.push(current_total);
            } else if let Some(min) = top_3_totals.iter_mut().min() {
                if *min < current_total {
                    *min = current_total;
                }
            }
            current_total = 0;
        } else {
            current_total += str::parse::<i32>(line)?;
        }
    }
    Ok(top_3_totals.iter().sum())
}

#[test]
fn test() {
    assert_eq!(
        solve_part_1(read_lines("input/d1_test.txt").unwrap()).unwrap(),
        24000
    );
    assert_eq!(
        solve_part_1(read_lines("input/d1_1_real.txt").unwrap()).unwrap(),
        70296
    );
    assert_eq!(
        solve_part_2(read_lines("input/d1_test.txt").unwrap()).unwrap(),
        45000
    );
    assert_eq!(
        solve_part_2(read_lines("input/d1_2_real.txt").unwrap()).unwrap(),
        205381
    );
}
