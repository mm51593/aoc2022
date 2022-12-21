use std::{collections::LinkedList, io::{stdin, Stdin, Lines, StdinLock}};

const ROUNDS: u32 = 20;
const DIVISOR: u32 = 3;

struct Monkey {
    items: LinkedList<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    divisible_by: u32,
    if_true: usize,
    if_false: usize,
    item_count: u32,
}

impl Monkey {
    pub fn take_item(&mut self, item: u32) {
        self.items.push_back(item);
    }

    pub fn operate(&self, item: u32) -> u32 {
        (self.operation)(item)
    }
}

pub fn run() {
    let mut lines = stdin().lines();

    let mut monkeys = Vec::new();
    loop {
        monkeys.push(parse_input(&mut lines));

        if let Some(_) = lines.next() { break; }
    }

    for _ in 0..ROUNDS {
        for monkey in monkeys.iter() {
            for item in monkey.items.into_iter() {
                let new_item = monkey.operate(item) / DIVISOR;
                if new_item % monkey.divisible_by == 0 {
                    monkeys.get_mut(monkey.if_true).unwrap().take_item(new_item);
                }
            }
        }
    }
}

fn parse_input(lines: &mut Lines<StdinLock>) -> Monkey
{
    let _ = strip_next(lines, "Monkey");
    
    let items = parse_items(split_input(&strip_next(lines, "  Starting items: ")));
    let operation = parse_operation(split_input(&strip_next(lines, "  Operation: new = ")));
    let divisible_by = parse_next(split_input(&strip_next(lines, "  Test: divisible by ")));
    let if_true = parse_next(split_input(&strip_next(lines, "    If true: throw to monkey ")));
    let if_false = parse_next(split_input(&strip_next(lines, "    If false: throw to monkey ")));

    Monkey { items: items, operation:operation, divisible_by: divisible_by, if_true: if_true as usize, if_false: if_false as usize, item_count: 0}
}

fn strip_next(lines: &mut Lines<StdinLock>, prefix: &str) -> String {
    lines.next().unwrap().unwrap().strip_prefix(prefix).unwrap().to_owned()
}

fn split_input<'a>(input: &'a String) -> Vec<&'a str> {
    input.split(" ")
        .collect::<Vec<_>>()
}

fn parse_items(input: Vec<&str>) -> LinkedList<u32> {
    input
        .into_iter()
        .map(|x| x
            .replace(",", "")
            .parse::<u32>()
            .unwrap())
        .collect::<LinkedList<_>>()
}

fn parse_operation(input: Vec<&str>) -> Box<dyn Fn(u32) -> u32> {
    match input[..] {
        ["old", "*", "old"] => Box::new(|x| x * x),
        ["old", "+", "old"] => Box::new(|x| x + x),
        ["old", "*", y] => {
            let y = y.parse::<u32>().unwrap();
            Box::new(move |x| x * y)
        }
        ["old", "+", y] => {
            let y = y.parse::<u32>().unwrap();
            Box::new(move |x| x + y)
        }
        _ => panic!(),
    }
}

fn parse_next(input: Vec<&str>) -> u32 {
    input.first().unwrap().parse::<u32>().unwrap()
}