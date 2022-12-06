use std::error::Error;

type Range = std::ops::RangeInclusive<i32>;

fn parse_range_pairs(input: &str) -> Option<Vec<(Range, Range)>> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let mut ranges = line.split(',').filter_map(|range_str| {
                    let mut ints = range_str.split('-').filter_map(|v| v.parse::<i32>().ok());
                    Some(ints.next()?..=ints.next()?)
                });
                Some((ranges.next()?, ranges.next()?))
            })
            .collect(),
    )
}

fn ranges_fully_overlap((range1, range2): &&(Range, Range)) -> bool {
    (range1.start() >= range2.start() && range1.end() <= range2.end())
        || (range2.start() >= range1.start() && range2.end() <= range1.end())
}

fn ranges_partially_overlap((range1, range2): &&(Range, Range)) -> bool {
    range1.contains(range2.start())
        || range1.contains(range2.end())
        || range2.contains(range1.start())
        || range2.contains(range1.end())
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(parse_range_pairs(input)
        .ok_or("Failed to parse range pairs.")?
        .iter()
        .filter(ranges_fully_overlap)
        .count()
        .to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(parse_range_pairs(input)
        .ok_or("Failed to parse range pairs.")?
        .iter()
        .filter(ranges_partially_overlap)
        .count()
        .to_string())
}
