use std::{io::stdin, ops::RangeInclusive};

#[derive(Debug)]
struct TimeStamp {
    time: u32,
    register: i32,
}

struct Node<'a> {
    lower: u32,
    upper: u32,
    left_child: Child<'a>,
    right_child: Child<'a>,
}

enum Child<'a> {
    N(Box<Node<'a>>),
    TS(&'a TimeStamp),
}

pub fn run() {
    let mut time = 1;
    let mut register = 1;

    let timestamps = stdin().lines().flatten()
        .flat_map(|s| {
            let mut split = s.split(" ");
            match split.next().unwrap() {
                "addx" => {
                    time += 2;
                    register += split.next().unwrap().parse::<i32>().unwrap();
                    Some(TimeStamp { time: time, register: register })
                },
                "noop" => {
                    time += 1;
                    None
                },
                _ => panic!()
            }
        }).collect::<Vec<_>>();

    println!("{:?}", timestamps);
    let tree = build_tree(&timestamps, 0, timestamps.len() as u32);
    //dfs(&tree, 0);
    let res = RangeInclusive::new(20, 220).step_by(40)
        .map(|i| (i, find(&tree, i)))
        .map(|p| p.0 * p.1 as u32)
        .sum::<u32>();

    println!("{:?}", res);
}

fn build_tree(timestamps: &Vec<TimeStamp>, lower: u32, upper: u32) -> Box<Node> {
    if upper - lower == 2 {
        let left_child = timestamps.get(lower as usize).unwrap();
        let right_child = timestamps.get((lower + 1) as usize).unwrap();
        Box::new(Node {
            lower: left_child.time,
            upper: right_child.time,
            left_child: Child::TS(left_child),
            right_child: Child::TS(right_child),
        })
    }
    else {
        let left_child = if (upper - lower) / 2 == 1 {
            Child::TS(timestamps.get(lower as usize).unwrap()) 
        } else {
            Child::N(build_tree(timestamps, lower, lower + (upper - lower) / 2))
        };

        let right_child = Child::N(build_tree(timestamps, lower + (upper - lower) / 2, upper));

        let lower_bound = match left_child {
            Child::N(ref n) => n.lower,
            Child::TS(ref t) => t.time,
        };

        let upper_bound = match right_child {
            Child::N(ref n) => n.upper,
            Child::TS(ref t) => t.time,
        };

        Box::new(Node {lower: lower_bound, upper: upper_bound, left_child: left_child, right_child: right_child})
    }
}

fn dfs(tree: &Box<Node>, depth: u32) {
    match &tree.left_child {
        Child::N(n) => { println!("{} {} {}", depth, n.lower, n.upper); dfs(&n, depth + 1) },
        Child::TS(t) => println!("{} {:?}", depth, t),
    }

    match &tree.right_child {
        Child::N(n) =>  { println!("{} {} {}", depth, n.lower, n.upper); dfs(&n, depth + 1) },
        Child::TS(t) => println!("{} {:?}", depth, t),
    }
}

fn find(tree: &Box<Node>, time: u32) -> i32 {
    if time >= get_lower_bound(&tree.right_child) {
        match &tree.right_child {
            Child::N(n) => find(&n, time),
            Child::TS(t) => t.register,
        }
    }
    else {
        match &tree.left_child {
            Child::N(n) => find(&n, time),
            Child::TS(t) => t.register,
        }
    }
}

fn get_lower_bound(x: &Child) -> u32 {
    match x {
        Child::N(n) => n.lower,
        Child::TS(t) => t.time,
    }
}