use std::{io::stdin, collections::HashSet};

const NODE_COUNT: usize = 10;

#[derive(Copy, Clone)]
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
    let mut nodes = Vec::new();

    for _ in 0..NODE_COUNT {
        nodes.push(Position { x: 0, y: 0 });
    }

    let mut visited = HashSet::new();
    visited.insert((nodes.last().unwrap().x, nodes.last().unwrap().y));

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
            move_head(&mut nodes, &dir, &mut visited);
        }
    }

    println!("{}", visited.len());
}

pub fn move_head(nodes: &mut Vec<Position>, direction: &Direction, visited: &mut HashSet<(i32, i32)>) {
    let mut head = nodes.get_mut(0).unwrap();
    match direction {
        Direction::U => head.y += 1,
        Direction::D => head.y -= 1,
        Direction::L => head.x -= 1,
        Direction::R => head.x += 1,
    }

    for i in 1..nodes.len() {
        let first = nodes[i - 1];
        let second = nodes.get_mut(i).unwrap();
        if i32::abs(first.x - second.x) >= 2 {
            second.x += (first.x - second.x) / i32::abs(first.x - second.x);
            if first.y - second.y != 0 {
                second.y += (first.y - second.y) / i32::abs(first.y - second.y);
            }
        }
        else if i32::abs(first.y - second.y) >= 2 {
            second.y += (first.y - second.y) / i32::abs(first.y - second.y);
            if first.x - second.x != 0 {
                second.x += (first.x - second.x) / i32::abs(first.x - second.x);
            }
        }
    }
    visited.insert((nodes.last().unwrap().x, nodes.last().unwrap().y));
}