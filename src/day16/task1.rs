use std::{io::stdin, collections::HashMap, cmp::Ordering};
use regex::Regex;
use lazy_static::lazy_static;

const TRAVEL_TIME: u32 = 1;
const OPENING_TIME: u32 = 1;
const TOTAL_TIME: u32 = 30;
const STARTING_NODE: &str = "AA";

#[derive(Debug)]
struct Node {
    rate: u32,
    adjacent: Vec<usize>,
}

pub fn run() {
    let (nodes, active_nodes, start) = get_input();

    let distances = floyd_warshall(&nodes);

    //let res = find_best(&mut active_nodes, *nodes, &distances, start);
    
    let res = dfs(&active_nodes, &distances, &nodes, start);
    println!("{res}");
}

fn get_input() -> (HashMap<usize, Node>, Vec<usize>, usize) {
    let mut nodes = HashMap::new();
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
            if cap[1].to_owned() == STARTING_NODE { start = Some(index)}
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

fn floyd_warshall(nodes: &HashMap<usize, Node>) -> Vec<Vec<u32>> {
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
    
    let mut grid = floyd_warshall_setup(&nodes);
    floyd_warshall_calc(&mut grid);
    grid
}

fn heuristic(n1: usize, n2: usize, start: usize, distances: &[Vec<u32>], nodes: &HashMap<usize, Node>) -> Ordering {
    let node1 = nodes.get(&n1).unwrap();
    let node2 = nodes.get(&n2).unwrap();

    let node1_rank = node1.rate / distances[start][n1];
    let node2_rank = node2.rate / distances[start][n2];

    node2_rank.cmp(&node1_rank)
}

fn dfs(active_nodes: &[usize], distances: &[Vec<u32>], nodes: &HashMap<usize, Node>, start: usize) -> u32 {
    // prepare nodes
    let mut working_order = active_nodes.to_owned();
    working_order.sort_unstable_by(|n1, n2| heuristic(*n1, *n2, start, distances, nodes));
    let mut visited = (0..working_order.len()).map(|_| false).collect::<Vec<_>>();

    let total_rate = nodes.iter().map(|f| f.1.rate).reduce(|acc, elem| acc + elem).unwrap();

    fn dfs_step(
            working_order: &Vec<usize>,
            distances: &[Vec<u32>],
            nodes: &HashMap<usize, Node>,
            previous_node: usize,
            visited: &mut Vec<bool>,
            time_remaining: u32,
            sum: u32,
            best_so_far: &mut u32,
            rate_remaining: u32) -> u32 {
        let mut best = 0;
        for i in 0..working_order.len() {
            if !visited[i] {
                visited[i] = true;

                let current_node = working_order[i];
                let dist = distances[previous_node][current_node];
                let new_time = match time_remaining.checked_sub(dist + OPENING_TIME) {
                    Some(x) => x,
                    None => {
                        visited[i] = false;
                        continue
                    }
                };
                let rate = nodes.get(&current_node).unwrap().rate * new_time;

                // prune
                let new_rate = rate_remaining - nodes.get(&current_node).unwrap().rate;
                if (sum + rate) + (time_remaining - OPENING_TIME - TRAVEL_TIME) * rate_remaining < *best_so_far {
                    visited[i] = false;
                    continue;
                }

                best  = u32::max(best, dfs_step(working_order, distances, nodes, current_node, visited, new_time, sum + rate, best_so_far, new_rate));
                visited[i] = false;
            }
        }
        
        let ret = u32::max(best, sum);
        if ret > *best_so_far { *best_so_far = ret; }
        ret
    }

    dfs_step(&working_order, distances, nodes, start, &mut visited, TOTAL_TIME, 0, &mut 0, total_rate)
}