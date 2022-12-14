use std::io::stdin;

const LETTER_COUNT: usize = 26;

pub fn run() {
    let sum = stdin().lines()
        .filter(|x| x.as_ref().unwrap().len() != 0)
        .map(|x| {
            let mut count = [0; LETTER_COUNT * 2];
            let half = x.as_ref().unwrap().len() / 2;
            for letter in x.as_ref().unwrap().as_bytes().iter().enumerate() {
                let priorioty = *letter.1 as u32 - u32::from('A');

                let index = if priorioty > u32::from('Z') - u32::from('A') {
                    *letter.1 as u32 - u32::from('a') + 1
                }
                else {
                    priorioty + 27
                } as usize;

                if letter.0 < half {
                    count[index - 1] += 1;
                }
                else {
                    if count[index - 1] > 0 { return index as u32 }
                }
            }
            return 0;
        })
        .sum::<u32>();

    println!("{}", sum);
}