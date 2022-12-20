use std::{io::stdin, collections::HashSet};

pub struct Position {
    x: i32,
    y: i32,
}

pub enum Direction {
    U,
    D,
    L,
    R,
}

pub fn run() {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut visited = HashSet::new();
    visited.insert((tail.x, tail.y));

    for line in stdin().lines().flatten() {
        let mut split = line.split(" ");
        let dir = match split.next().unwrap() {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => panic!()
        };
        let count = split.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..count {
            move_head(&mut head, &mut tail, &dir, &mut visited);
        }
    }

    println!("{}", visited.len());
}

pub fn move_head(head: &mut Position, tail: &mut Position, direction: &Direction, visited: &mut HashSet<(i32, i32)>) {
    match direction {
        Direction::U => head.y += 1,
        Direction::D => head.y -= 1,
        Direction::L => head.x -= 1,
        Direction::R => head.x += 1,
    }

    // tail needs updating?
    if i32::abs(head.x - tail.x) >= 2 {
        tail.x += (head.x - tail.x) / i32::abs(head.x - tail.x);
        if head.y - tail.y != 0 {
            tail.y += head.y - tail.y;
        }
    }
    else if i32::abs(head.y - tail.y) >= 2 {
        tail.y += (head.y - tail.y) / i32::abs(head.y - tail.y);
        if head.x - tail.x != 0 {
            tail.x += head.x - tail.x;
        }
    }
    visited.insert((tail.x, tail.y));
}