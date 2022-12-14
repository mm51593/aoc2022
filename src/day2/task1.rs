use std::io::stdin;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

pub fn run() {
    let res = stdin()
        .lines()
        .map(|x| {
            if x.as_ref().unwrap().len() == 0 {
                0
            }
            else {
                let temp = x.as_ref().unwrap().as_bytes();
                let first = (temp[0] - 65) as i32;
                let second = (temp[2] - 88) as i32;

                match first - second {
                    -2 => second + 1 + LOSS,
                    -1 => second + 1 + WIN,
                    0 => second + 1 + DRAW, 
                    1 => second + 1 + LOSS,
                    2 => second + 1 + WIN,
                    _ => panic!(),
                }
            }
        }).reduce(|accum, item| accum + item);
    
    println!("{}", res.unwrap());
}