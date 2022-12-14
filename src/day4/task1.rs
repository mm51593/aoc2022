use std::io::stdin;

pub fn run() {
    let input = stdin().lines().flatten().collect::<Vec<_>>();
    let pairs = input.iter()
        .map(|l| l.split(",")
            .map(|x| x.split("-")
                .flat_map(|n| n.parse::<usize>())
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let sum = pairs.iter()
        .map(|p| {
            (p[0][0] <= p[1][0] && p[0][1] >= p[1][1] || 
            p[1][0] <= p[0][0] && p[1][1] >= p[0][1]) as usize
        }).sum::<usize>();

    println!("{}", sum);
}