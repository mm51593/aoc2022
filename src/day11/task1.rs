use std::{collections::LinkedList, io::{stdin, Lines, StdinLock}};

const ROUNDS: u32 = 20;
const DIVISOR: u64 = 3;
const ACTIVITY_TOP: usize = 2;

struct Monkey {
    items: LinkedList<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    item_count: u32,
}

impl Monkey {
    pub fn take_item(&mut self, item: u64) {
        self.items.push_back(item);
    }

    pub fn operate(&self, item: u64) -> u64 {
        (self.operation)(item)
    }

    pub fn add_to_throw_list(&self, item: u64, list: &mut LinkedList<(usize, u64)>) {
        let item = self.operate(item) / DIVISOR;
        let destination = if item % self.divisible_by == 0 { self.if_true } else { self.if_false };
        list.push_back((destination, item));
    }
}

pub fn run() {
    let mut lines = stdin().lines();

    let mut monkeys = Vec::new();
    loop {
        monkeys.push(parse_input(&mut lines));

        if let None = lines.next() { break; } 
    }

    for _ in 0..ROUNDS {
        for monkey in 0..monkeys.len() {
            let mut throw_list = LinkedList::new();
            for item in monkeys[monkey].items.iter() {
                monkeys[monkey].add_to_throw_list(*item, &mut throw_list);
            }

            for (dest, item) in throw_list {
                monkeys[dest].take_item(item);
                monkeys[monkey].item_count += 1;
            }

            monkeys[monkey].items = LinkedList::new();
        }
    }

    let mut activity = monkeys.iter()
        .map(|x| x.item_count)
        .collect::<Vec<_>>();

    activity.sort_by(|a, b| a.cmp(b).reverse());

    let res = activity[..ACTIVITY_TOP]
        .iter()
        .copied()
        .reduce(|acc, item| acc * item)
        .unwrap();

    println!("{}", res);
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

fn parse_items(input: Vec<&str>) -> LinkedList<u64> {
    input
        .into_iter()
        .map(|x| x
            .replace(",", "")
            .parse::<u64>()
            .unwrap())
        .collect::<LinkedList<_>>()
}

fn parse_operation(input: Vec<&str>) -> Box<dyn Fn(u64) -> u64> {
    match input[..] {
        ["old", "*", "old"] => Box::new(|x| x * x),
        ["old", "+", "old"] => Box::new(|x| x + x),
        ["old", "*", y] => {
            let y = y.parse::<u64>().unwrap();
            Box::new(move |x| x * y)
        }
        ["old", "+", y] => {
            let y = y.parse::<u64>().unwrap();
            Box::new(move |x| x + y)
        }
        _ => panic!(),
    }
}

fn parse_next(input: Vec<&str>) -> u64 {
    input.first().unwrap().parse::<u64>().unwrap()
}