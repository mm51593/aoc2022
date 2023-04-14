use std::{io::stdin, cmp, fmt::Debug};

const COUNT: u32 = 2022;
const WIDTH: usize = 7;

const VERTICAL_OFFSET: usize = 3;
const HORIZONTAL_OFFSET: usize = 2;

struct Shape {
    points: Vec<(isize, isize)>,
    height: isize,
}

impl Shape {
    fn new(points: Vec<(isize, isize)>) -> Shape {
        let (mut min, mut max) = points[0];
        points.iter().for_each(|(_w, h)| {
            if *h < min {
                min = *h;
            }
            else if *h > max {
                    max = *h
            }
        });

        Shape { points: points, height: max - min + 1 }
    }
}

#[derive(Clone, Copy)]
enum BlockType {
    Full,
    Empty,
    Edge,
}

impl Debug for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => write!(f, "#"),
            Self::Empty => write!(f, "."),
            Self::Edge => write!(f, "|"),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

// ####
const HLINE_POINTS: [(isize, isize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];

// .#.
// ###
// .#.

const PLUS_POINTS: [(isize, isize); 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];

// ..#
// ..#
// ###

const L_POINTS: [(isize, isize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];

// #
// #
// #
// #

const VLINE_POINTS: [(isize, isize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];

// ##
// ##
const BOX_POINTS: [(isize, isize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

pub fn run() {
    let shapes = vec![
        Shape::new(HLINE_POINTS.to_vec()),
        Shape::new(PLUS_POINTS.to_vec()),
        Shape::new(L_POINTS.to_vec()),
        Shape::new(VLINE_POINTS.to_vec()),
        Shape::new(BOX_POINTS.to_vec())
    ];

    let wind = stdin().lines().next().unwrap().unwrap()
        .as_bytes()
        .into_iter()
        .map(|c| match c {
            b'>' => Direction::Right,
            b'<' => Direction::Left,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let mut height = 0;
    let mut field = Vec::new();

    let mut wind_iter = wind.iter().cycle();
    let mut shape_iter = shapes.iter().cycle();

    for _ in 0..COUNT {
        while field.len() < height + VERTICAL_OFFSET + 1 {
            field.push([BlockType::Empty; WIDTH]);
        }
        //print_field(&field);
        let shape = shape_iter.next().unwrap();
        let (mut x, mut y) = (HORIZONTAL_OFFSET as isize, field.len() as isize - 1);

        let mut count = 0;
        loop {
            let dir = wind_iter.next().unwrap();
            match dir {
                Direction::Left => {
                    if check_edges(&shape.points, (x - 1, y), &field) {
                        x -= 1;
                    }
                },
                Direction::Right => {
                    if check_edges(&shape.points, (x + 1, y), &field) {
                        x += 1;
                    }
                }
            }

            if count < 2 || check_bottom(&shape.points, (x, y - 1), &field) {
                y -= 1;
            }
            else {
                draw_shape(&shape.points, (x, y), &mut field);
                height = cmp::max(height, (y + shape.height).try_into().unwrap());
                break;
            }
            count += 1;
        }
    }

    println!("{height}");
}

fn check_edges(shape_points: &Vec<(isize, isize)>, position: (isize, isize), field: &Vec<[BlockType; WIDTH]>) -> bool {
    let (x, y) = position;
    shape_points.iter().flat_map(|(px, py)| {
        field.get(TryInto::<usize>::try_into(y + py).ok()?)
            .and_then(|r| {
                match TryInto::<usize>::try_into(x + px).ok().and_then(|i| r.get(i)) {
                    Some(x) => Some(x),
                    None => Some(&BlockType::Edge),
                }
            })
    }).map(|f| match f {
        BlockType::Full => false,
        BlockType::Empty => true,
        BlockType::Edge => false,
    }).reduce(|acc, elem| acc && elem).unwrap()
}

fn check_bottom(shape_points: &Vec<(isize, isize)>, position: (isize, isize), field: &Vec<[BlockType; WIDTH]>) -> bool {
    let (x, y) = position;
    shape_points.iter().flat_map(|(px, py)| {
        match TryInto::<usize>::try_into(py + y).ok().and_then(|r| field.get(r)) {
            Some(row) => {
                row.get((px + x) as usize)
            }
            None => Some(&BlockType::Edge),
        }
    }).map(|f| match f {
        BlockType::Full => false,
        BlockType::Empty => true,
        BlockType::Edge => false,
    }).reduce(|acc, elem| acc && elem).unwrap()
}

fn draw_shape(shape_points: &Vec<(isize, isize)>, position: (isize, isize), field: &mut Vec<[BlockType; WIDTH]>) {
    let (x, y) = position;
    //println!("{} {}", x, y);
    shape_points.iter().for_each(|p| {
        field[(y + p.1) as usize][(x + p.0) as usize] = BlockType::Full;
    })
}

fn _print_field(field: &Vec<[BlockType; WIDTH]>) {
    for row in field.iter().rev() {
        for elem in row {
            print!("{:?}", elem);
        }
        println!()
    }
    println!()
}