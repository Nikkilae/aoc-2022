use std::{cmp::max, error::Error};

struct ForestGrid {
    values: Vec<Vec<i8>>,
    width: i32,
    height: i32,
}
impl ForestGrid {
    fn parse_from_input(input: &str) -> Self {
        let mut values = Vec::new();
        let mut max_width = 0;
        for line in input.lines() {
            let mut row = Vec::new();
            for height in line.chars().map(|c| c as i8 - 48) {
                row.push(height);
            }
            max_width = max(max_width, row.len());
            values.push(row);
        }
        Self {
            width: max_width as i32,
            height: values.len() as i32,
            values,
        }
    }

    fn get(&self, x: i32, y: i32) -> i8 {
        if let Some(row) = self.values.get(y as usize) {
            return *row.get(x as usize).unwrap_or(&-1);
        }
        -1
    }

    fn is_visible(&self, x: i32, y: i32) -> bool {
        let h = self.get(x, y);
        (0..y).all(|yy| self.get(x, yy) < h)
            || (y + 1..self.height).all(|yy| self.get(x, yy) < h)
            || (0..x).all(|xx| self.get(xx, y) < h)
            || (x + 1..self.width).all(|xx| self.get(xx, y) < h)
    }

    fn get_scenic_score(&self, x: i32, y: i32) -> usize {
        let h = self.get(x, y);
        let mut score_left = (0..x).rev().take_while(|xx| self.get(*xx, y) < h).count();
        let mut score_right = (x + 1..self.width)
            .take_while(|xx| self.get(*xx, y) < h)
            .count();
        let mut score_up = (0..y).rev().take_while(|yy| self.get(x, *yy) < h).count();
        let mut score_down = (y + 1..self.height)
            .take_while(|yy| self.get(x, *yy) < h)
            .count();
        if (score_left as i32) < x {
            score_left += 1;
        }
        if (score_right as i32) < self.width - 1 - x {
            score_right += 1;
        }
        if (score_up as i32) < y {
            score_up += 1;
        }
        if (score_down as i32) < self.height - 1 - y {
            score_down += 1;
        }
        score_left * score_right * score_up * score_down
    }
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = ForestGrid::parse_from_input(input);
    let mut n = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.is_visible(x, y) {
                n += 1;
            }
        }
    }
    Ok(n.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let grid = ForestGrid::parse_from_input(input);
    let mut max_score = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let score = grid.get_scenic_score(x, y);
            max_score = max(score, max_score);
        }
    }
    Ok(max_score.to_string())
}
