use std::{collections::HashMap, io::stdin, iter::Peekable};

#[derive(Debug)]
enum Node {
    D(Vec<String>, Option<String>),
    F(u32, String)
}

const DISK_SIZE: u32 = 70_000_000;
const SPACE_REQUIRED: u32 = 30_000_000;

pub fn run() {
    let lines = stdin().lines()
        .flatten().filter(|s| s.len() > 0).collect::<Vec<_>>();
        
    let mut tree = HashMap::new();

    let mut pwd = "/".to_string();
    tree.insert("/".to_string(), Node::D(Vec::new(), None));

    let mut iter = lines.iter().peekable();
    while let Some(cmd) = iter.next() {
        let cmd_split = cmd.split(" ").collect::<Vec<_>>();
        match cmd_split[1] {
            "cd" => {
                pwd = match cmd_split[2] {
                    ".." => match tree.get(&pwd).unwrap() {
                        Node::D(_, p) => p.as_ref().unwrap().to_string(),
                        Node::F(_, p) => p.to_string(),
                    },
                    "/" => "/".to_string(),
                    d => {
                        let mut name = String::from(pwd);
                        name.push('/');
                        name.push_str(d);
                        name
                    },
                }; ()
            }
            "ls" => {
                match tree.remove(&pwd).unwrap() {
                    Node::D(_, p) => {
                        let new_node = Node::D(list_directory(&mut tree, &mut iter, &pwd), p.to_owned());
                        tree.insert(pwd.clone(), new_node);
                    },
                    _ => ()
                }
                ()
            },
            _ => panic!(),
        }
    }

    println!("{:?}", get_result(&tree).unwrap());
}

fn list_directory<'a, I: Iterator<Item = &'a String>>(
    tree: &mut HashMap<String, Node>,
    iter: &mut Peekable<I>,
    parent: &String
) -> Vec<String> {
    let mut listing = Vec::new();
    while let Some(line) = iter.peek() {
        if line.starts_with("$") { break; }
        let line_split = line.split(" ").collect::<Vec<_>>();
        let mut name = String::from(parent);
        name.push('/');
        name.push_str(line_split[1]);
        listing.push(name.to_owned());
        match line_split[0] {
            "dir" => tree.insert(name, Node::D(Vec::new(), Some(parent.to_owned()))),
            _ => tree.insert(name, Node::F(line_split[0].parse().unwrap(), parent.to_owned())),
        };

        iter.next();
    }
    listing
}

fn get_result(tree: &HashMap<String, Node>) -> Option<u32> {
    let disk_usage = calculate_size(&"/".to_string(), &tree);
    let disk_remaining = DISK_SIZE - disk_usage;
    let to_free = SPACE_REQUIRED - disk_remaining;

    tree.iter()
        .filter(|(key, _)| {
            match tree.get(*key).unwrap() {
                Node::D(_, _) => true,
                Node::F(_, _) => false,
            }
        }).map(|(key, _)| calculate_size(key, tree))
        .filter(|s| *s >= to_free)
        .reduce(|accum, item| std::cmp::min(accum, item))
}

fn calculate_size(node: &String, tree: &HashMap<String, Node>) -> u32 {
    match tree.get(node).unwrap() {
        Node::D(c, _) => c.iter().map(|n| calculate_size(n, tree)).sum(),
        Node::F(s, _) => *s
    }
}