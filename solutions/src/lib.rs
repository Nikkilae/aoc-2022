use std::error::Error;

pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;

pub struct Day {
    pub name: String,
    pub solution: Option<Solution>,
}
impl Day {
    fn new(name: &str, solution: Option<Solution>) -> Self {
        Self {
            name: name.to_string(),
            solution,
        }
    }
}
pub struct Solution {
    pub source_code: String,
    pub solve_part_1: fn(&str) -> Result<String, Box<dyn Error>>,
    pub solve_part_2: fn(&str) -> Result<String, Box<dyn Error>>,
}
impl Solution {
    fn new(
        source_code: String,
        solve_part_1: fn(&str) -> Result<String, Box<dyn Error>>,
        solve_part_2: fn(&str) -> Result<String, Box<dyn Error>>,
    ) -> Self {
        Self {
            source_code,
            solve_part_1,
            solve_part_2,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref DAYS: Vec<Day> = vec![
        Day::new("Calorie Counting", Some(Solution::new(include_str!("d01.rs").to_string(), d01::solve_part_1, d01::solve_part_2))),
        Day::new("Rock Paper Scissors", Some(Solution::new(include_str!("d02.rs").to_string(), d02::solve_part_1, d02::solve_part_2))),
        Day::new("Rucksack Reorganization", Some(Solution::new(include_str!("d03.rs").to_string(), d03::solve_part_1, d03::solve_part_2))),
        Day::new("Camp Cleanup", Some(Solution::new(include_str!("d04.rs").to_string(), d04::solve_part_1, d04::solve_part_2))),
        Day::new("Supply Stacks", Some(Solution::new(include_str!("d05.rs").to_string(), d05::solve_part_1, d05::solve_part_2))),
        Day::new("Tuning Trouble", Some(Solution::new(include_str!("d06.rs").to_string(), d06::solve_part_1, d06::solve_part_2))),
        Day::new("No Space Left On Device", Some(Solution::new(include_str!("d07.rs").to_string(), d07::solve_part_1, d07::solve_part_2))),
        Day::new("Treetop Tree House", Some(Solution::new(include_str!("d08.rs").to_string(), d08::solve_part_1, d08::solve_part_2))),
        Day::new("Rope Bridge", Some(Solution::new(include_str!("d09.rs").to_string(), d09::solve_part_1, d09::solve_part_2))),
        Day::new("Cathode-Ray Tube", Some(Solution::new(include_str!("d10.rs").to_string(), d10::solve_part_1, d10::solve_part_2))),
        Day::new("Monkey in the Middle", Some(Solution::new(include_str!("d11.rs").to_string(), d11::solve_part_1, d11::solve_part_2))),
        Day::new("Hill Climbing Algorithm", Some(Solution::new(include_str!("d12.rs").to_string(), d12::solve_part_1, d12::solve_part_2))),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
        Day::new("???", None),
    ];
}
