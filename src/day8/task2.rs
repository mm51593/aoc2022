use std::io::stdin;

#[derive(Debug)]
struct Cell {
    height: u8,
    scenic: u32,
}

impl Cell {
    pub fn update_scenic(&mut self, multiplier: u32)
    {
        self.scenic *= multiplier;
    }
}

struct CellIndex<'a> {
    cell: &'a mut Cell,
    index: usize,
}

pub fn run() {
    let mut chart = stdin().lines().flatten()
        .map(|x| x.as_bytes().iter()
            .map(|y| Cell {height: *y, scenic: 1})
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    println!("{}", get_max_scenic(&mut chart));
}

fn get_max_scenic(chart: &mut Vec<Vec<Cell>>) -> u32 {
    // left - right
    for line in chart.iter_mut() {
        let mut stack: Vec<CellIndex> = Vec::new();
        let line_len = line.len();
        for (i, cell) in line.iter_mut().enumerate() {
            if stack.len() == 0 || stack.last().unwrap().cell.height > cell.height{
            }
            else {
                while stack.len() != 0 && stack.last().unwrap().cell.height <= cell.height {
                    let top = stack.pop().unwrap();
                    top.cell.update_scenic((i - top.index) as u32);
                }
            }
            stack.push(CellIndex { cell: cell, index: i})
        }
        while let Some(top) = stack.pop() {
            top.cell.update_scenic((line_len - top.index - 1) as u32);
        }
    }

    let line_width = chart[0].len();
    // right - left
    for line in chart.iter_mut() {
        let mut stack: Vec<CellIndex> = Vec::new();
        for (index, cell) in line.iter_mut().rev().enumerate() {
            let i = line_width - index - 1;
            if stack.len() == 0 || stack.last().unwrap().cell.height > cell.height{
            }
            else {
                while stack.len() != 0 && stack.last().unwrap().cell.height <= cell.height {
                    let top = stack.pop().unwrap();
                    top.cell.update_scenic((top.index - i) as u32);
                }
            }
            stack.push(CellIndex { cell: cell, index: i})
        }
        while let Some(top) = stack.pop() {
            top.cell.update_scenic((top.index) as u32);
        }
    }

    /* transpose grid */
    let chart_len = chart.len();
    let mut iters = chart.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();
    let mut transposed = (0..chart_len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap()).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    // left - right
    for line in transposed.iter_mut() {
        let mut stack: Vec<CellIndex> = Vec::new();
        let line_len = line.len();
        for (i, cell) in line.iter_mut().enumerate() {
            if stack.len() == 0 || stack.last().unwrap().cell.height > cell.height{
            }
            else {
                while stack.len() != 0 && stack.last().unwrap().cell.height <= cell.height {
                    let top = stack.pop().unwrap();
                    top.cell.update_scenic((i - top.index) as u32);
                }
            }
            stack.push(CellIndex { cell: cell, index: i})
        }
        while let Some(top) = stack.pop() {
            top.cell.update_scenic((line_len - top.index - 1) as u32);
        }
    }

    let line_width = transposed[0].len();
    let mut max_scenic = 0;
    // right - left
    for line in transposed.iter_mut() {
        let mut stack: Vec<CellIndex> = Vec::new();
        for (index, cell) in line.iter_mut().rev().enumerate() {
            let i = line_width - index - 1;
            if stack.len() == 0 || stack.last().unwrap().cell.height > cell.height{
            }
            else {
                while stack.len() != 0 && stack.last().unwrap().cell.height <= cell.height {
                    let top = stack.pop().unwrap();
                    top.cell.update_scenic((top.index - i) as u32);
                    max_scenic = std::cmp::max(max_scenic, top.cell.scenic);
                }
            }
            stack.push(CellIndex { cell: cell, index: i})
        }
        while let Some(top) = stack.pop() {
            top.cell.update_scenic((top.index) as u32);
            max_scenic = std::cmp::max(max_scenic, top.cell.scenic);
        }
    }

    max_scenic
}