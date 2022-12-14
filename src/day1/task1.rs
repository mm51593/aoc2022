use std::io::stdin;

pub fn run() {
    let mut sum = 0;

    let mut max = 0;

    for line in stdin().lines() {
        match line.unwrap().as_str() {
            "" => {
                if sum > max { max = sum; }
                sum = 0;
            }
            s => {
                sum += s.parse::<i64>().unwrap();
            }
        }
    }

    println!("{}", max);
}
