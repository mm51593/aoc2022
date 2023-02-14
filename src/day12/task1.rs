use std::{io::stdin, collections::LinkedList, convert::TryInto};

pub fn run() {
    let mut grid = stdin().lines().flatten()
        .filter(|s| !s.is_empty())
        .map(|x| x.as_bytes()
            .iter().map(|c| c.to_owned())
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let mut start = (None, None);
    let mut end = (None, None);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'S' as u8 {
                start = (Some(i), Some(j));
            }
            else if grid[i][j] == b'E' as u8 {
                end = (Some(i), Some(j));
            }
        }
        if let (Some(_), Some(_)) = start {
            if let (Some(_), Some(_)) = end {
                break;
            }
        }
    }


    let start = (start.0.unwrap(), start.1.unwrap());
    let end = (end.0.unwrap(), end.1.unwrap());
    grid[end.0][end.1] = b'z' as u8;

    println!("{:?} {:?}", start, end);

    let res = bfs(&grid, start, end);
    println!("{}", res.unwrap());
}

fn bfs(grid: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> Option<u64> {
    let mut grid_visited = grid.iter()
        .map(|v| v.iter()
            .map(|_| None)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    // initialise search
    let mut queue = LinkedList::new();
    queue.push_back(((start.0, start.1), b'a' as u8));
    grid_visited[start.0][start.1] = Some(0);

    // search
    while let Some((pos, height)) = queue.pop_front() {
        let dist = grid_visited[pos.0][pos.1].unwrap();
        println!("{:?} {} {}", pos, height, dist);
        match pos {
            _ if pos == end => {
                return grid_visited[pos.0][pos.1]
            },
            (y, x) => [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .flat_map(|(dy, dx)| Some((
                    TryInto::<usize>::try_into(y as isize + dy).ok()?,
                    TryInto::<usize>::try_into(x as isize + dx).ok()?)))
                .filter(|&(dy, dx)| dy < grid.len() && dx < grid[0].len())
                .filter(|&(dy, dx)| grid[dy][dx] <= height + 1)
                .for_each(|(dy, dx)| if grid_visited[dy][dx].is_none() {
                    grid_visited[dy][dx] = Some(dist + 1);
                    queue.push_back(((dy, dx), grid[dy][dx]));
                })
        }
    }

    None
}