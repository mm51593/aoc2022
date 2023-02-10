use std::{io::stdin, collections::HashMap, ops::RangeInclusive};

const SOURCE: (i32, i32) = (500, 0);

#[derive(Debug)]
enum BlockType {
    Rock,
    Sand,
}

pub fn run() {
    let input = stdin().lines().flatten()
        .map(|s| s.split(" -> ")
            .map(|t| t.to_owned())
            .map(|t| construct_position(t))
            .collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let mut map = HashMap::new();

    let limit = input.iter()
        .map(|x| construct_map(&mut map, x))
        .reduce(|acc, elem| std::cmp::max(acc, elem)).unwrap();

    let mut count = 0;
    while drop_sand(&mut map, SOURCE, limit) {
        count += 1;
    }

    println!("{}", count);
}

fn construct_position(input: String) -> (i32, i32) {
    let temp = input.split(",").collect::<Vec<_>>();
    (temp[0].parse().unwrap(), temp[1].parse().unwrap())
}

fn construct_map(map: &mut HashMap<(i32, i32), BlockType>, path: &Vec<(i32, i32)>) -> i32 {
    let mut deepest = 0;
    let mut prev: Option<(i32, i32)> = None;

    for elem in path {
        deepest = std::cmp::max(deepest, elem.1);
        match prev {
            Some(prev) if prev.0 == elem.0 => {
                let (c1, c2) = if prev.1 < elem.1 { (prev.1, elem.1) } else { (elem.1, prev.1) };
                for i in RangeInclusive::new(c1, c2) {
                    map.insert((prev.0, i), BlockType::Rock);
                }
            },
            Some(prev) => {
                let (c1, c2) = if prev.0 < elem.0 { (prev.0, elem.0) } else { (elem.0, prev.0) };
                for i in RangeInclusive::new(c1, c2) {
                    map.insert((i, prev.1), BlockType::Rock);
                }
            },
            None => {
                map.insert(*elem, BlockType::Rock);
            },
        }
        prev = Some(*elem);
    }
    deepest
}

fn drop_sand(map: &mut HashMap<(i32, i32), BlockType>, position: (i32, i32), limit: i32) -> bool {
    if position.1 >= limit { return false; }

    let adjacent = [(position.0, position.1 + 1), (position.0 - 1, position.1 + 1), (position.0 + 1, position.1 + 1)];
    let new_pos = adjacent.iter().find(|x| map.get(x).is_none());

    match new_pos {
        Some(x) => drop_sand(map, *x, limit),
        None => {
            map.insert(position, BlockType::Sand);
            true
        },
    }
}