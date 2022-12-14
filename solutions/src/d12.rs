use std::{collections::HashMap, error::Error};

use priority_queue::DoublePriorityQueue;

type Pos = (i32, i32);

struct Grid {
    values: Vec<Vec<char>>,
}
impl Grid {
    fn parse_from_input(input: &str) -> Option<(Self, Pos, Pos)> {
        let mut values = Vec::new();
        let (mut start, mut end) = (None, None);
        for (y, line) in input.lines().enumerate() {
            let row = line.chars().collect::<Vec<char>>();
            start = start.or_else(|| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, c)| (*c == 'S').then_some((x as i32, y as i32)))
            });
            end = end.or_else(|| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, c)| (*c == 'E').then_some((x as i32, y as i32)))
            });
            values.push(row);
        }
        Some((Self { values }, start?, end?))
    }

    fn get_char(&self, x: i32, y: i32) -> Option<char> {
        if let Some(row) = self.values.get(y as usize) {
            return row.get(x as usize).copied();
        }
        None
    }

    fn get_height(&self, x: i32, y: i32) -> Option<i32> {
        self.get_char(x, y).map(|c| match c {
            'S' => 0,
            'E' => 'z' as i32 - 'a' as i32,
            _ => c as i32 - 'a' as i32,
        })
    }
}

fn neighbors(grid: &Grid, (x, y): Pos, inverse: bool) -> Vec<Pos> {
    let Some(here) = grid.get_height(x, y) else {
        return Vec::new();
    };
    static DIFFS: &[Pos; 4] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    DIFFS
        .iter()
        .map(|(xd, yd)| (x + xd, y + yd))
        .filter(|(xx, yy)| {
            let Some(h) = grid.get_height(*xx, *yy) else { return false; };
            if inverse {
                here - h <= 1
            } else {
                h - here <= 1
            }
        })
        .collect()
}

fn reconstruct_path_len(to_node: &Pos, came_from: &HashMap<Pos, Pos>) -> usize {
    let mut len = 0;
    let mut node = to_node;
    while let Some(cf) = came_from.get(node) {
        len += 1;
        node = cf;
    }
    len
}

fn find_path(grid: &Grid, start: &Pos, end: &Pos) -> Option<usize> {
    // Find the shortest path from start to end with A*.

    fn heuristic((x1, y1): &Pos, (x2, y2): &Pos) -> i32 {
        (x2 - x1).abs() + (y2 - y1).abs()
    }

    let mut open: DoublePriorityQueue<Pos, i32> = DoublePriorityQueue::new();
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g: HashMap<Pos, i32> = HashMap::new();

    g.insert(*start, 0);
    open.push(*start, heuristic(start, end));

    while let Some((current_node, _)) = open.pop_min() {
        if current_node == *end {
            return Some(reconstruct_path_len(end, &came_from));
        }
        for neighbor in neighbors(grid, current_node, false) {
            let tg = *g.get(&current_node)? + 1;
            let neighbor_g = g.get(&neighbor).copied();
            if neighbor_g.is_none() || tg < neighbor_g.unwrap() {
                came_from.insert(neighbor, current_node);
                g.insert(neighbor, tg);
                open.push(neighbor, tg + heuristic(&neighbor, end));
            }
        }
    }
    None
}

fn find_path_to_closest_ground(grid: &Grid, end: &Pos) -> Option<usize> {
    // Find the closest path from end to any zero-height node.
    // It's just A* with the heuristic being a constant value (0) so I guess it's equivalent to Dijsktra's?

    let mut open: DoublePriorityQueue<Pos, i32> = DoublePriorityQueue::new();
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g: HashMap<Pos, i32> = HashMap::new();

    g.insert(*end, 0);
    open.push(*end, 0);

    while let Some((current_node, _)) = open.pop_min() {
        if grid.get_height(current_node.0, current_node.1) == Some(0) {
            return Some(reconstruct_path_len(&current_node, &came_from));
        }
        for neighbor in neighbors(grid, current_node, true) {
            let tg = *g.get(&current_node)? + 1;
            let neighbor_g = g.get(&neighbor).copied();
            if neighbor_g.is_none() || tg < neighbor_g.unwrap() {
                came_from.insert(neighbor, current_node);
                g.insert(neighbor, tg);
                open.push(neighbor, tg);
            }
        }
    }
    None
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let (grid, start, end) = Grid::parse_from_input(input).ok_or("Failed to parse the grid.")?;
    let path_length = find_path(&grid, &start, &end).ok_or("Failed to find a path.")?;
    Ok(path_length.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let (grid, _, end) = Grid::parse_from_input(input).ok_or("Failed to parse the grid.")?;
    let path_len = find_path_to_closest_ground(&grid, &end).ok_or("Failed to find any path.")?;
    Ok(path_len.to_string())
}
