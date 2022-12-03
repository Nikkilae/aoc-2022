use std::error::Error;

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

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|line| {
            let (compartment1, compartment2) = line.split_at(line.len() / 2);
            sum_of_priorities(&[compartment1, compartment2])
        })
        .sum::<i32>()
        .to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(sum_of_priorities)
        .sum::<i32>()
        .to_string())
}
