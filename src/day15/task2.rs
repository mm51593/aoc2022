use std::{io::stdin, collections::HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

const X_MIN: i32 = 0;
const X_MAX: i32 = 4000000;
const Y_MIN: i32 = 0;
const Y_MAX: i32 = 4000000;

const MULTIPLIER: i64 = 4000000;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    a: i32,
    b: i32,
}

pub fn run() {
    let mut regions = Vec::new();
    let mut lines_positive = Vec::new();
    let mut lines_negative = Vec::new();
    get_input().for_each(|(s, b)| {
        let manh = i32::abs(s.x - b.x) + i32::abs(s.y - b.y);

        // y = ax + b
        // b = y - ax

        // x = s.x +- manh
        lines_positive.push(Line { a: 1, b: s.y - (s.x + manh) });
        lines_positive.push(Line { a: 1, b: s.y - (s.x - manh) });

        lines_negative.push(Line { a: -1, b: s.y + (s.x + manh) });
        lines_negative.push(Line { a: -1, b: s.y + (s.x - manh) });

        regions.push((s, manh));
    });


    let mut fragments = HashSet::new();
    for i in 0..lines_positive.len() {
        for j in i + 1..lines_positive.len() {
            for k in 0..lines_negative.len() {
                for l in k + 1..lines_negative.len() {
                    let pos = [&lines_positive[i], &lines_positive[j]];
                    let neg = [&lines_negative[k], &lines_negative[l]];
                    let points = itertools::iproduct!(pos, neg)
                        .map(|(p, n)| intersect_lines(p, n))
                        .flatten()
                        .collect::<Vec<_>>();

                    if points.len() != pos.len() * neg.len() {
                        continue;
                    }

                    let found = points.iter().combinations(2)
                        .map(|v| {
                            let a = v[0];
                            let b = v[1];

                            i32::abs(a.x - b.x) + i32::abs(a.y - b.y) == 2
                        })
                        .reduce(|acc, elem| acc && elem).unwrap();

                    if found {
                        let point = get_median(&points);

                        if point.x >= X_MIN && point.x <= X_MAX &&
                            point.y >= Y_MIN && point.y <= Y_MAX 
                        {
                            fragments.insert(point);
                        }
                    }
                }
            }
        }
    }

    for frag in fragments {
        let mut found = true;
        for (s, manh) in regions.iter() {
            let dist = i32::abs(s.x - frag.x) + i32::abs(s.y - frag.y);
            if dist < *manh { 
                found = false;
                break;
            }
        }

        if found {
            println!("{}", frag.x as i64 * MULTIPLIER + frag.y as i64);
        }
    }
}

fn get_input() -> impl Iterator<Item=(Position, Position)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)$").unwrap();
    }

    stdin().lines().flatten()
        .map(|f| {
            let cap = RE.captures(&f).unwrap();

            (Position { x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap() },
             Position { x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap() })
        })
}

fn intersect_lines(l1: &Line, l2: &Line) -> Option<Position> {
    // a1x + b1 = a2x + b2
    // a1x - a2x = b2 - b1
    // (a1 - a2)x = b2 - b1
    // x = (b2 - b1) / (a1 - a2)

    let lhs = l1.a - l2.a;
    let rhs = l2.b - l1.b;

    let x = rhs / lhs;

    if x as f32 - (rhs as f32 / lhs as f32) > 0.001 {
        return None;
    }

    let y = l1.a * x + l1.b;

    
    Some(Position { x, y })
}

fn get_median(points: &Vec<Position>) -> Position {
    let total = points.iter()
        .map(|x| (x.x, x.y))
        .reduce(|acc, elem| (acc.0 + elem.0, acc.1 + elem.1)).unwrap();

    Position { x: total.0 / points.len() as i32, y: total.1 / points.len() as i32 }
}