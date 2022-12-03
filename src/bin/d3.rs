use std::error::Error;

use aoc_2022::read_lines;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", solve_part_1(read_lines("input/d3_real.txt")?));
    println!("Part 2: {}", solve_part_2(read_lines("input/d3_real.txt")?));
    Ok(())
}

fn find_common_characters<T: AsRef<str>>(strings: &[T]) -> Vec<char> {
    if strings.is_empty() {
        return Vec::new();
    }
    strings[0]
        .as_ref()
        .char_indices()
        .filter_map(|(i, c)| (strings[0].as_ref().find(c) == Some(i)).then_some(c))
        .filter(|c| {
            strings[1..]
                .iter()
                .all(|string| string.as_ref().contains(*c))
        })
        .collect()
}

fn sum_of_priorities<T: AsRef<str>>(strings: &[T]) -> i32 {
    find_common_characters(strings)
        .into_iter()
        .map(|c| {
            if ('a'..='z').contains(&c) {
                c as i32 - 96
            } else {
                c as i32 - 38
            }
        })
        .sum::<i32>()
}

fn solve_part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> i32 {
    lines
        .map(|line| {
            let line = line.as_ref();
            let (compartment1, compartment2) = line.split_at(line.len() / 2);
            sum_of_priorities(&[compartment1, compartment2])
        })
        .sum()
}

fn solve_part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> i32 {
    lines
        .map(|line| line.as_ref().to_string())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(sum_of_priorities)
        .sum()
}

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    assert_eq!(solve_part_1(read_lines("input/d3_test.txt")?), 157);
    assert_eq!(solve_part_1(read_lines("input/d3_real.txt")?), 8243);
    assert_eq!(solve_part_2(read_lines("input/d3_test.txt")?), 70);
    assert_eq!(solve_part_2(read_lines("input/d3_real.txt")?), 2631);
    Ok(())
}
