use std::{error::Error, iter::once};

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut max_total = 0;
    let mut current_total = 0;
    for line in input.lines().chain(once("")) {
        if line.is_empty() {
            if current_total > max_total {
                max_total = current_total;
            }
            current_total = 0;
        } else {
            current_total += str::parse::<i32>(line)?;
        }
    }
    Ok(max_total.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut top_3_totals = Vec::new();
    let mut current_total = 0;
    for line in input.lines().chain(once("")) {
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
    Ok(top_3_totals.iter().sum::<i32>().to_string())
}
