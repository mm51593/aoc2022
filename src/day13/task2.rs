use core::panic;
use std::{io::stdin, cmp::Ordering};

const OPEN_BRACE: u8 = b'[' as u8;
const CLOSE_BRACE: u8 = b']' as u8;
const COMMA: u8 = b',' as u8;

#[derive(Clone)]
enum PackageElem {
    PList(Vec<PackageElem>),
    PInt(i32),
}

pub fn run() {
    let mut packages = Vec::new();
    let divider1 = PackageElem::PList(vec![PackageElem::PList(vec![PackageElem::PInt(2)])]);
    let divider2 = PackageElem::PList(vec![PackageElem::PList(vec![PackageElem::PInt(6)])]);

    packages.push(divider1.clone());
    packages.push(divider2.clone());

    let mut iter = stdin().lines();
    while let Some(Ok(left)) = iter.next() {
        let right = iter.next().unwrap().unwrap();

        let left_package = parse_package(left.as_bytes(), &mut 0).unwrap();
        let right_package = parse_package(right.as_bytes(), &mut 0).unwrap();

        packages.push(left_package);
        packages.push(right_package);

        let _ = iter.next();
    }

    packages.sort_unstable_by(compare_packages);

    let product = packages.iter().enumerate().filter_map(|(i, x)|
        if compare_packages(x, &divider1) == Ordering::Equal ||
        compare_packages(x, &divider2) == Ordering::Equal {
            Some(i + 1)
        }
        else {
            None
        }
    ).reduce(|acc, elem| acc * elem);

    println!("{}", product.unwrap());
}

fn parse_package(line: &[u8], index: &mut usize) -> Option<PackageElem> {
    let to_parse = line[*index];

    if to_parse == OPEN_BRACE {
        _ = get_next(line, index);
        let mut elements = Vec::new();
        loop {
            match parse_package(line, index) {
                Some(x) => elements.push(x),
                None => (),
            }
            match get_next(line, index) {
                COMMA => (),
                CLOSE_BRACE => break,
                _ => panic!(),
            }
        }
        Some(PackageElem::PList(elements))
    }
    else {
        let mut num = None;
        while line[*index] != COMMA as u8 && line[*index] != CLOSE_BRACE as u8 {
            let temp = num.unwrap_or_default() * 10 + (get_next(line, index) - b'0' as u8) as i32;
            num = Some(temp);
        }
        num.map(PackageElem::PInt)
    }
}

fn get_next(array: &[u8], index: &mut usize) -> u8 {
    let ret = array[*index];
    *index += 1;
    ret
}

fn compare_packages(left: &PackageElem, right: &PackageElem) ->  Ordering {
    match (left, right) {
        (PackageElem::PList(l), PackageElem::PList(r)) => {
            for i in 0..l.len() {
                if i >= r.len() {
                    return Ordering::Greater;
                }
                match compare_packages(l.get(i).unwrap(), r.get(i).unwrap()) {
                    Ordering::Equal => (),
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                }
            }
            if l.len() == r.len() {
                Ordering::Equal
            }
            else {
                Ordering::Less
            }
        },
        (PackageElem::PList(_), PackageElem::PInt(r)) =>  compare_packages(left, &PackageElem::PList(vec![PackageElem::PInt(*r)])),
        (PackageElem::PInt(l), PackageElem::PList(_)) => compare_packages(&PackageElem::PList(vec![PackageElem::PInt(*l)]), right),
        (PackageElem::PInt(l), PackageElem::PInt(r)) => {
            match l - r {
                i if i < 0 => Ordering::Less,
                i if i == 0 => Ordering::Equal,
                _ => Ordering::Greater,
            }
        },
    }
}