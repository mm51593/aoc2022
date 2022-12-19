use std::io::stdin;

#[derive(Debug)]
struct Cell {
    height: u8,
    counted: bool,
}

pub fn run() {
    let mut chart = stdin().lines().flatten()
        .map(|x| x.as_bytes().iter()
            .map(|y| Cell {height: *y, counted: false})
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    println!("{}", count_visible(&mut chart));
}

fn count_visible(chart: &mut Vec<Vec<Cell>>) -> i32 {
    let mut count = 0;

    // left - right
    for line in chart.iter_mut() {
        let mut highest = 0;
        for cell in line {
            if cell.height > highest {
                highest = cell.height;
                if !cell.counted {
                    count += 1;
                    cell.counted = true
                }
            }
        }
    }

    // right - left
    for line in chart.iter_mut() {
        let mut highest = 0;
        for cell in line.iter_mut().rev() {
            if cell.height > highest {
                highest = cell.height;
                if !cell.counted {
                    count += 1;
                    cell.counted = true
                }
            }
        }
    }

    let width = chart[0].len();

    // up - down
    for i in 0..width {
        let mut highest = 0;
        for j in 0..chart.len() {
            let mut cell = &mut chart[j][i];
            if cell.height > highest {
                highest = cell.height;
                if !cell.counted {
                    count += 1;
                    cell.counted = true
                }
            }
        }
    }

    // down - up
    for i in 0..width {
        let mut highest = 0;
        for j in (0..chart.len()).rev() {
            let mut cell = &mut chart[j][i];
            if cell.height > highest {
                highest = cell.height;
                if !cell.counted {
                    count += 1;
                    cell.counted = true
                }
            }
        }
    }

    count
}