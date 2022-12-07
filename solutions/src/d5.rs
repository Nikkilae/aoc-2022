use std::error::Error;

use regex::Regex;

type Stack = Vec<char>;
type Action = (usize, usize, usize); // (n, from, to)

fn do_actions_part_1(stacks: &mut [Stack], (n, from, to): &Action) -> Option<()> {
    for _ in 0..*n {
        let char = stacks.get_mut(*from)?.pop()?;
        stacks.get_mut(*to)?.push(char);
    }
    Some(())
}

fn do_actions_part_2(stacks: &mut [Stack], (n, from, to): &Action) -> Option<()> {
    let from_stack = stacks.get_mut(*from)?;
    let mut moved_chars = from_stack.drain(from_stack.len() - n..).collect();
    stacks.get_mut(*to)?.append(&mut moved_chars);
    Some(())
}

fn parse_stacks(input: &str) -> Option<Vec<Stack>> {
    let mut stacks = Vec::new();
    for line in input.lines() {
        if !line.contains('[') {
            break;
        }
        let mut chars = line.chars();
        for i in 0.. {
            chars.next();
            if stacks.len() <= i {
                stacks.push(Vec::new())
            }
            let char = chars.next()?;
            if char != ' ' {
                stacks[i].insert(0, char);
            }
            chars.next();
            if chars.next().is_none() {
                break;
            }
        }
    }
    Some(stacks)
}

fn parse_actions(input: &str) -> Result<Vec<Action>, Box<dyn Error>> {
    let regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$")?;
    let mut moves = Vec::new();
    for line in input.lines() {
        if let Some(caps) = regex.captures(line) {
            let n = caps[1].parse::<usize>()?;
            let from = caps[2].parse::<usize>()? - 1;
            let to = caps[3].parse::<usize>()? - 1;
            moves.push((n, from, to));
        }
    }
    Ok(moves)
}

fn get_stack_tops(stacks: &[Stack]) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut stacks = parse_stacks(input).unwrap();
    for action in &parse_actions(input)? {
        do_actions_part_1(&mut stacks, action).unwrap();
    }
    Ok(get_stack_tops(&stacks))
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut stacks = parse_stacks(input).unwrap();
    for action in &parse_actions(input)? {
        do_actions_part_2(&mut stacks, action).unwrap();
    }
    Ok(get_stack_tops(&stacks))
}
