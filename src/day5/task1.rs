use std::{io::stdin, fmt::Error, char};

pub fn run() {
    let lines = stdin().lines().flatten().collect::<Vec<_>>();

    let separator_index = match find_separator(&lines) {
        Ok(s) => s,
        Err(_) => panic!(),
    };

    let mut stacks = setup_stacks(&lines, separator_index);

    let moves = (separator_index + 1..lines.len())
        .map(|i| &lines[i])
        .filter(|f| f.len() > 0)
        .map(|s| parse_command(&s))
        .collect::<Vec<_>>();

    for mv in moves {
        execute_move(&mut stacks, mv.0, mv.1, mv.2);
    }

    println!("{:?}", get_result(&mut stacks));
}

fn find_separator(lines: &Vec<String>) -> Result<usize, Error> {
    for (index, line) in lines.iter().enumerate() {
        if line == "" {
            return Ok(index)
        }
    }
    Err(Error)
}

fn extract_characters(line: &String) -> Vec<char>
{
    let mut characters = Vec::new();
    let line_bytes = line.as_bytes();

    let mut i = 0;
    while i * 4 < line.len() {
        characters.push(char::from(line_bytes[i * 4 + 1]));

        i += 1;
    }

    characters
}

fn setup_stacks(lines: &Vec<String>, separator_index: usize) -> Vec<Vec<char>> {
    let axis_vec = extract_characters(&lines[separator_index - 1]);
    let mut stacks = (0..axis_vec.len()).map(|_| Vec::new()).collect::<Vec<_>>();

    for i in (0..separator_index - 1).rev() {
        let chars = extract_characters(&lines[i]);
        for (index, to_add) in chars.into_iter().enumerate() {
            match to_add {
                ' ' => (),
                s => stacks[index].push(s),
            }
        }
    }

    stacks
}

fn parse_command(command: &String) -> (usize, usize, usize) {
    let cmd_split = command.split(" ").collect::<Vec<_>>();
    (cmd_split[1].parse::<usize>().unwrap(),
    cmd_split[3].parse::<usize>().unwrap() - 1,
    cmd_split[5].parse::<usize>().unwrap() - 1)
}

fn execute_move(stacks: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) {
    for _ in 0..count {
        let obj = stacks[from].pop();
        match obj {
            Some(s) => stacks[to].push(s),
            None => panic!(),
        }
    }
}

fn get_result(stacks: &mut Vec<Vec<char>>) -> Vec<char> {
    stacks.iter_mut()
        .map(|s| s.pop().unwrap())
        .collect::<Vec<_>>()
}