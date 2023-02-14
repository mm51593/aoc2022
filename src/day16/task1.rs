use std::{io::stdin, collections::HashMap, cmp::Ordering};
use itertools::{Permutations, Itertools};
use regex::Regex;
use lazy_static::lazy_static;

const TRAVEL_TIME: u32 = 1;
const OPENING_TIME: u32 = 1;

#[derive(Debug)]
struct Node {
    rate: u32,
    adjacent: Vec<usize>,
}

pub fn run() {
    let (nodes, mut active_nodes, start) = get_input();

    let distances = floyd_warshall(&nodes);

    active_nodes.sort_unstable_by(|n1, n2| heuristic(*n1, *n2, start, &distances, &nodes));
    permutate(active_nodes);
}

fn get_input() -> (Box<HashMap<usize, Node>>, Vec<usize>, usize) {
    let mut nodes = Box::new(HashMap::new());
    let mut active_nodes = Vec::new();
    let mut lut = HashMap::new();

    lazy_static! {
        static ref RE: Regex = Regex::new("Valve (.+) has flow rate=(.+); tunnels? leads? to valves? (.+)$").unwrap();
    }
    let mut count: usize = 0;
    let mut start = None;
    stdin().lines().flatten()
        .for_each(|f| {
            let cap = RE.captures(&f).unwrap();

            let index = match lut.get(&cap[1].to_owned()) {
                Some(x) => *x,
                None => {
                    lut.insert(cap[1].to_owned(), count);
                    count += 1;
                    count - 1
                },
            };
            if cap[1].to_owned() == "AA" { start = Some(index)}
            nodes.insert(index, Node { rate: cap[2].parse::<u32>().unwrap(), 
                adjacent: cap[3].split(", ").map(|f| {
                    if lut.get(f).is_none() {
                        lut.insert(f.to_owned(), count);
                        count += 1;
                    }
                    *lut.get(f).unwrap()
                }).collect::<Vec<_>>()});
            if cap[2].parse::<i32>().unwrap() > 0 { active_nodes.push(index);}
        });

    (nodes, active_nodes, start.unwrap())
}

fn floyd_warshall(nodes: &Box<HashMap<usize, Node>>) -> Vec<Vec<u32>> {
    let mut grid = floyd_warshall_setup(&nodes);
    floyd_warshall_calc(&mut grid);
    grid
}
fn floyd_warshall_setup(nodes: &HashMap<usize, Node>) -> Vec<Vec<u32>> {
    let mut grid = (0..nodes.len())
        .map(|_| (0..nodes.len())
            .map(|_| u32::MAX).collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    
    for (index, elem) in nodes.iter() {
        for neighbour in elem.adjacent.iter() {
            grid[*index][*neighbour] = TRAVEL_TIME;
        }
    }
    grid
}

fn floyd_warshall_calc(grid: &mut Vec<Vec<u32>>) {
    for k in 0..grid.len() {                // outer
        for i in 0..grid.len() {            // inner
            if i == k { continue; }
            for j in 0..grid.len() {        // iterator
                if j == k { continue; }
                grid[i][j] = u32::min(grid[i][j], 
                    grid[i][k].checked_add(grid[k][j]).map_or(u32::MAX, |x| x));
            }
        }
    }
}

fn heuristic(n1: usize, n2: usize, start: usize, distances: &Vec<Vec<u32>>, nodes: &HashMap<usize, Node>) -> Ordering {
    let node1 = nodes.get(&n1).unwrap();
    let node2 = nodes.get(&n2).unwrap();

    let node1_rank = node1.rate / distances[start][n1];
    let node2_rank = node2.rate / distances[start][n2];

    node2_rank.cmp(&node1_rank)
}

fn permutate(starting_point: Vec<usize>) {
    starting_point.iter().permutations(starting_point.len()).for_each(|f| println!("{f:?}"));
}