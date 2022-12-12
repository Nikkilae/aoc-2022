use std::error::Error;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    next_divider: usize,
    next_true_monkey: usize,
    next_false_monkey: usize,
}

fn parse_monkey(input: &str) -> Option<Monkey> {
    let lines = input.lines().collect::<Vec<&str>>();
    if lines.len() != 6 {
        return None;
    }
    let items = lines[1][18..]
        .split(", ")
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    let operator = lines[2].chars().nth(23)?;
    let operand = lines[2].get(25..)?.to_string();
    let operation = Box::new(move |old: usize| {
        let value = match operand.parse::<usize>() {
            Ok(value) => value,
            Err(_) => old,
        };
        match operator {
            '+' => old + value,
            '*' => old * value,
            _ => old,
        }
    });
    let next_divider = lines[3].get(21..)?.parse::<usize>().ok()?;
    let next_true_monkey = lines[4].get(29..)?.parse::<usize>().ok()?;
    let next_false_monkey = lines[5].get(30..)?.parse::<usize>().ok()?;

    Some(Monkey {
        items,
        operation,
        next_divider,
        next_true_monkey,
        next_false_monkey,
    })
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut monkeys = input
        .split("\r\n\r\n")
        .filter_map(parse_monkey)
        .map(Some)
        .collect::<Vec<Option<Monkey>>>();
    let mut num_inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].take().unwrap();
            for item in &monkey.items {
                let mut worry = (monkey.operation)(*item);
                worry /= 3;
                let next = if worry % monkey.next_divider == 0 {
                    monkey.next_true_monkey
                } else {
                    monkey.next_false_monkey
                };
                monkeys[next as usize].as_mut().unwrap().items.push(worry);
            }
            num_inspections[i] += monkey.items.len();
            monkey.items.clear();
            monkeys[i].replace(monkey);
        }
    }
    num_inspections.sort();
    let level_of_monkey_business =
        num_inspections[num_inspections.len() - 2] * num_inspections[num_inspections.len() - 1];
    Ok(level_of_monkey_business.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut monkeys = input
        .split("\r\n\r\n")
        .filter_map(parse_monkey)
        .map(Some)
        .collect::<Vec<Option<Monkey>>>();
    let mut num_inspections = vec![0; monkeys.len()];
    let m = monkeys
        .iter()
        .flatten()
        .map(|m| m.next_divider)
        .product::<usize>();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].take().unwrap();
            for item in &monkey.items {
                let mut worry = (monkey.operation)(*item);
                worry %= m;
                let next = if worry % monkey.next_divider == 0 {
                    monkey.next_true_monkey
                } else {
                    monkey.next_false_monkey
                };
                monkeys[next].as_mut().unwrap().items.push(worry);
            }
            num_inspections[i] += monkey.items.len();
            monkey.items.clear();
            monkeys[i].replace(monkey);
        }
    }
    num_inspections.sort();
    let level_of_monkey_business =
        num_inspections[num_inspections.len() - 2] * num_inspections[num_inspections.len() - 1];
    Ok(level_of_monkey_business.to_string())
}
