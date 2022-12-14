use std::io::stdin;
use std::collections::BinaryHeap;

const N: i32 = 3;

pub fn run() {
    let mut heap = BinaryHeap::<i64>::new();
    let mut sum = 0;


    for line in stdin().lines() {
        match line.unwrap().as_str() {
            "" => {
                heap.push(sum);
                sum = 0;
            }
            s => {
                sum += s.parse::<i64>().unwrap();
            }
        }
    }

    let mut total = 0;
    for _i in 0..N {
        total += heap.pop().unwrap();
    }

    println!("{}", total)
}
