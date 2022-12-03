use std::error::Error;

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
impl RPS {
    fn wins_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
    fn draws_against(&self) -> RPS {
        *self
    }
    fn loses_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
    fn base_score(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

fn rps_score((opponent, you): (RPS, RPS)) -> i32 {
    you.base_score()
        + (if you.wins_against() == opponent {
            6
        } else if you.draws_against() == opponent {
            3
        } else {
            0
        })
}

fn parse_line_part_1<T: AsRef<str>>(line: T) -> Option<(RPS, RPS)> {
    let mut rpss = line.as_ref().split(' ').filter_map(|str| match str {
        "A" | "X" => Some(RPS::Rock),
        "B" | "Y" => Some(RPS::Paper),
        "C" | "Z" => Some(RPS::Scissors),
        _ => None,
    });
    match (rpss.next(), rpss.next()) {
        (Some(opponent), Some(you)) => Some((opponent, you)),
        _ => None,
    }
}

fn parse_line_part_2<T: AsRef<str>>(line: T) -> Option<(RPS, RPS)> {
    let mut split = line.as_ref().split(' ');
    let opponent = split.next().and_then(|char| match char {
        "A" => Some(RPS::Rock),
        "B" => Some(RPS::Paper),
        "C" => Some(RPS::Scissors),
        _ => None,
    })?;
    let you = match split.next() {
        Some("X") => Some(opponent.wins_against()),
        Some("Y") => Some(opponent.draws_against()),
        Some("Z") => Some(opponent.loses_against()),
        _ => None,
    }?;
    Some((opponent, you))
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input
        .lines()
        .filter_map(parse_line_part_1)
        .map(rps_score)
        .sum::<i32>()
        .to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input
        .lines()
        .filter_map(parse_line_part_2)
        .map(rps_score)
        .sum::<i32>()
        .to_string())
}
