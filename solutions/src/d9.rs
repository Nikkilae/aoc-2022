use std::{collections::HashSet, error::Error};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse_moves(input: &str) -> Option<Vec<(char, i32)>> {
    let mut moves = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(' ');
        let dir = parts.next()?.chars().next()?;
        let n = parts.next()?.parse::<i32>().ok()?;
        moves.push((dir, n));
    }
    Some(moves)
}

fn move_pos(pos: &Pos, dir: char) -> Pos {
    let mut head = *pos;
    match dir {
        'L' => head.x -= 1,
        'R' => head.x += 1,
        'U' => head.y -= 1,
        'D' => head.y += 1,
        _ => {}
    }
    head
}

fn follow_after_move(lead: &Pos, follower: &Pos) -> Pos {
    let mut follower = *follower;
    let diff_x = (lead.x - follower.x).abs();
    let diff_y = (lead.y - follower.y).abs();
    if diff_x + diff_y >= 3 {
        follower.x += (lead.x - follower.x).signum();
        follower.y += (lead.y - follower.y).signum();
    } else if diff_x >= 2 {
        follower.x += (lead.x - follower.x).signum();
    } else if diff_y >= 2 {
        follower.y += (lead.y - follower.y).signum();
    }
    follower
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut tail_visited_positions: HashSet<Pos> = HashSet::new();
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = head;
    for (dir, n) in parse_moves(input).ok_or("Failed to parse moves from input.")? {
        for _ in 0..n {
            head = move_pos(&head, dir);
            tail = follow_after_move(&head, &tail);
            tail_visited_positions.insert(tail);
        }
    }
    Ok(tail_visited_positions.len().to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut tail_visited_positions: HashSet<Pos> = HashSet::new();
    let mut knots = vec![Pos { x: 0, y: 0 }; 10];
    for (dir, n) in parse_moves(input).ok_or("Failed to parse moves from input.")? {
        for _ in 0..n {
            knots[0] = move_pos(&knots[0], dir);
            for i in 1..knots.len() {
                knots[i] = follow_after_move(&knots[i - 1], &knots[i]);
            }
            tail_visited_positions.insert(*knots.last().unwrap());
        }
    }
    Ok(tail_visited_positions.len().to_string())
}
