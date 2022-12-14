#[allow(dead_code)]
use std::io::stdin;

const SEQUENCE_LENGTH: u32 = 14;

pub fn run() {
    let mut signal = String::new();
    stdin().read_line(&mut signal).unwrap();

    let bytes = signal.as_bytes();

    let mut index = 0;
    while index < bytes.len() {
        let mut count = 0;
        for i in 0..SEQUENCE_LENGTH {
            for j in i..SEQUENCE_LENGTH {
                if bytes[index + j as usize] == bytes[index + i as usize] {
                    count += 1;
                }
            }
        }

        if count == SEQUENCE_LENGTH {
            break;
        }
        index += 1;
    }

    println!("{}", index + SEQUENCE_LENGTH as usize)
}