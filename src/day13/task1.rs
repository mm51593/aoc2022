use core::panic;
use std::io::stdin;

const OPEN_BRACE: u8 = '[' as u8;
const CLOSE_BRACE: u8 = ']' as u8;
const COMMA: u8 = ',' as u8;

#[derive(Debug)]
enum PackageElem {
    PList(Vec<PackageElem>),
    PInt(i32),
}

enum Comparison {
    GT,
    LT,
    EQ,
}

pub fn run() {
    let mut sum = 0;
    let mut index = 0;

    let mut iter = stdin().lines();
    while let Some(Ok(left)) = iter.next() {
        let right = iter.next().unwrap().unwrap();

        index += 1;

        let left_package = parse_package(left.as_bytes(), &mut 0).unwrap();
        let right_package = parse_package(right.as_bytes(), &mut 0).unwrap();


        sum += compare_packages(&left_package, &right_package) as i32 * index;
        let _ = iter.next();
    }

    println!("{}", sum);
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
            let temp = num.unwrap_or_default() * 10 + (get_next(line, index) - '0' as u8) as i32;
            num = Some(temp);
        }
        num.map(|x| PackageElem::PInt(x))
    }
}

fn get_next(array: &[u8], index: &mut usize) -> u8 {
    let ret = array[*index];
    *index += 1;
    ret
}

fn compare_packages(left: &PackageElem, right: &PackageElem) ->  Comparison {
    if let PackageElem::PInt(l) = left {
        if let PackageElem::PInt(r) = right {
            match l - r {
                i if i < 0 => Comparison::LT,
                i if i == 0 => Comparison::EQ,
                _ => Comparison::GT,
            }
        }
        else if let PackageElem::PList(_) = right {
            compare_packages(&PackageElem::PList(vec![PackageElem::PInt(*l)]), right)
        }
        else {
            panic!()
        }
    }
    else if let PackageElem::PList(l) = left {
        if let PackageElem::PInt(r) = right {
            compare_packages(left, &PackageElem::PList(vec![PackageElem::PInt(*r)]))
        }
        else if let PackageElem::PList(r) = right {
            for i in 0..l.len() {
                if i >= r.len() {
                    return Comparison::GT;
                }
                match compare_packages(l.get(i).unwrap(), r.get(i).unwrap()) {
                    Comparison::GT => return Comparison::GT,
                    Comparison::LT => return Comparison::LT,
                    Comparison::EQ => (),
                }
            }
            Comparison::LT
        }
        else {
            panic!()
        }
    }
    else {
        panic!()
    }
}