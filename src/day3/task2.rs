use std::io::stdin;

const LETTER_COUNT: usize = 26;
const GROUP_SIZE: usize = 3;

pub fn run() {
    let it = stdin().lines()
        .filter(|x| x.as_ref().unwrap().len() != 0)
        .map(|x| {
            let mut count = [0; LETTER_COUNT * 2];
            //let half = x.as_ref().unwrap().len() / 2;
            for letter in x.as_ref().unwrap().as_bytes().iter().enumerate() {
                let priorioty = *letter.1 as u32 - u32::from('A');

                let index = if priorioty > u32::from('Z') - u32::from('A') {
                    *letter.1 as u32 - u32::from('a') + 1
                }
                else {
                    priorioty + 27
                } as usize;

                
                count[index - 1] = 1;
            }
            count
        }).collect::<Vec<_>>();

    let mut sum = 0;
    for i in (0..it.len()).step_by(GROUP_SIZE) {
        for j in 0..(LETTER_COUNT * 2) {
            let count = (0..GROUP_SIZE)
                .map(|k| it
                    .get(i + k).unwrap()
                    .get(j).unwrap()
                )
                .sum::<u32>();

            if count >= 3 {
                sum += j + 1;
                break;
            }
        }
    }
    println!("{}", sum);
}