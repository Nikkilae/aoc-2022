use std::{collections::HashSet, error::Error};

fn find_packet_index(input: &str, packet_size: usize) -> Option<usize> {
    let chars = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();
    (0..chars.len()).find_map(|i| {
        let chars: HashSet<&char> = HashSet::from_iter(chars[i..i + packet_size].iter());
        (chars.len() == packet_size).then_some(i + packet_size)
    })
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(find_packet_index(input, 4)
        .ok_or("No start-of-packet marker found.")?
        .to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(find_packet_index(input, 14)
        .ok_or("No start-of-message marker found.")?
        .to_string())
}
