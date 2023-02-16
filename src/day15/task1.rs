use std::{io::stdin, collections::HashSet};

use lazy_static::lazy_static;
use regex::Regex;

const LINE_NUM: i32 = 2000000;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Coverage {
    lb: i32,
    ub: i32,
}

impl Coverage {
    fn adjust(&mut self, other: &Coverage) {
        if self.lb <= other.lb && self.ub >= other.lb {
            self.ub = other.lb - 1;
        }
        else if self.lb <= other.ub && self.ub >= other.ub {
            self.lb = other.ub + 1;
        }
    }

    fn count(&self) -> Option<i32> {
        match self.ub - self.lb {
            x if x < 0 => None,
            x => Some(x + 1),
        }
    }

    fn encopasses(&self, other: &Coverage) -> bool {
        self.lb <= other.lb && self.ub >= other.ub
    }
}

pub fn run() {
    let mut line_elements = Vec::new();
    let mut beacons_at_line = HashSet::new();
    get_input().for_each(|(s, b)| {
        if b.y == LINE_NUM {
            beacons_at_line.insert(b.x);
        }

        add_coverage_to_vector(coverage_at_line(s, b, LINE_NUM), &mut line_elements);
    });

    let line_sum = line_elements.iter()
        .map(|f| f.count())
        .flatten()
        .reduce(|acc, elem| acc + elem).unwrap();

    let beacons_covered = beacons_at_line.iter().map(|x| {
            line_elements.iter()
                .map(|f| *x >= f.lb && *x <= f.ub)
                .reduce(|acc, elem| acc || elem)
                .unwrap()
        }).map(|f| f as i32)
        .reduce(|acc, elem| acc + elem)
        .unwrap_or(0);

    let res = line_sum - beacons_covered;
    println!("{res}");
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

fn coverage_at_line(sensor: Position, beacon: Position, line: i32) -> Coverage {
    let manh = i32::abs(sensor.x - beacon.x) + i32::abs(sensor.y - beacon.y);

    let dist = manh - i32::abs(sensor.y - line);
    
    Coverage { lb: sensor.x - dist, ub: sensor.x + dist }
}

fn add_coverage_to_vector(mut cov: Coverage, cov_vec: &mut Vec<Coverage>) {
    if cov.count().is_none() { return; }

    let mut to_remove = Vec::new();
    for (idx, elem) in cov_vec.iter().enumerate() {
        if elem.encopasses(&cov) { return; }
        if cov.encopasses(elem) { to_remove.push(idx); continue; }

        cov.adjust(elem);
        if cov.count().is_none() { return; }
    }
    to_remove.into_iter().rev().for_each(|i| { cov_vec.remove(i); });

    cov_vec.push(cov);
}