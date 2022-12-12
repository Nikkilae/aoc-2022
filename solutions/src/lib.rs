use std::error::Error;

pub mod d1;
pub mod d10;
pub mod d11;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;

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
        Day::new("Calorie Counting", Some(Solution::new(include_str!("d1.rs").to_string(), d1::solve_part_1, d1::solve_part_2))),
        Day::new("Rock Paper Scissors", Some(Solution::new(include_str!("d2.rs").to_string(), d2::solve_part_1, d2::solve_part_2))),
        Day::new("Rucksack Reorganization", Some(Solution::new(include_str!("d3.rs").to_string(), d3::solve_part_1, d3::solve_part_2))),
        Day::new("Camp Cleanup", Some(Solution::new(include_str!("d4.rs").to_string(), d4::solve_part_1, d4::solve_part_2))),
        Day::new("Supply Stacks", Some(Solution::new(include_str!("d5.rs").to_string(), d5::solve_part_1, d5::solve_part_2))),
        Day::new("Tuning Trouble", Some(Solution::new(include_str!("d6.rs").to_string(), d6::solve_part_1, d6::solve_part_2))),
        Day::new("No Space Left On Device", Some(Solution::new(include_str!("d7.rs").to_string(), d7::solve_part_1, d7::solve_part_2))),
        Day::new("Treetop Tree House", Some(Solution::new(include_str!("d8.rs").to_string(), d8::solve_part_1, d8::solve_part_2))),
        Day::new("Rope Bridge", Some(Solution::new(include_str!("d9.rs").to_string(), d9::solve_part_1, d9::solve_part_2))),
        Day::new("Cathode-Ray Tube", Some(Solution::new(include_str!("d10.rs").to_string(), d10::solve_part_1, d10::solve_part_2))),
        Day::new("Monkey in the Middle", Some(Solution::new(include_str!("d11.rs").to_string(), d11::solve_part_1, d11::solve_part_2))),
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
        Day::new("???", None),
    ];
}
