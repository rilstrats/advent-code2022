use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(mut lines) = read_lines("data/input.txt") {
        // store all lines related to diagram in a vector
        let mut diagram = vec![];
        loop {
            let line = lines.next();
            if let Some(Ok(line)) = line {
                if line == "" {
                    break;
                } else {
                    diagram.push(line)
                }
            }
        }

        // reverse vector for easier processing and create iterable
        diagram.reverse();
        let mut diagram_lines = diagram.iter();

        // look at first line of diagram to find bay/character and index
        let line = diagram_lines.next().unwrap();
        let mut bay_dict: HashMap<char, usize> = HashMap::new();
        for (index, character) in line.chars().enumerate() {
            if character == ' ' {
                continue;
            }
            bay_dict.insert(character, index);
        }

        // loop through rest of lines to add cargo to dict
        let mut cargo_dict: HashMap<char, Vec<char>> = HashMap::new();
        for line in diagram_lines {
            for (bay, index) in bay_dict.iter() {
                let cargo = line.chars().nth(*index).unwrap();
                if cargo == ' ' {
                    continue;
                }
                cargo_dict.entry(*bay).or_insert(Vec::new()).push(cargo);
            }
        }

        // println!("{:#?}", cargo_dict)

        for line in lines {
            if let Ok(line) = line {
                // cargo_move(&line, &mut cargo_dict); // part one
                cargo_move2(&line, &mut cargo_dict); // part two
            }
        }

        for (bay, cargo) in cargo_dict {
            println!("{}: {}", bay, cargo.last().unwrap());
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn cargo_move(command: &str, dict: &mut HashMap<char, Vec<char>>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let amount: i32 = parts[1].parse().unwrap();
    let from = parts[3].chars().next().unwrap();
    let to = parts[5].chars().next().unwrap();

    for _ in 0..amount {
        let moving = dict.entry(from).or_insert(Vec::new()).pop().unwrap();
        dict.entry(to).or_insert(Vec::new()).push(moving);
    }
}

fn cargo_move2(command: &str, dict: &mut HashMap<char, Vec<char>>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let amount: i32 = parts[1].parse().unwrap();
    let from = parts[3].chars().next().unwrap();
    let to = parts[5].chars().next().unwrap();

    let mut moving: Vec<char> = vec![];
    for _ in 0..amount {
        moving.insert(0, dict.entry(from).or_insert(Vec::new()).pop().unwrap());
    }
    dict.entry(to).or_insert(Vec::new()).append(&mut moving);
}
