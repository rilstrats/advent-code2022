use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
    ops::AddAssign,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: String;
    if args.get(1).is_none() {
        file_path = String::from("data/input.txt");
    } else {
        file_path = format!("data/{}.txt", args.get(1).unwrap());
    }

    let cycles: Vec<i32> = (20..1000).step_by(40).collect();
    let mut values: Vec<i32> = vec![];

    let mut instructions: Vec<Instruction> = vec![];

    let lines = read_lines(file_path).expect("Error reading file");
    for line in lines {
        let line = line.expect("Error reading line");
        let instruction = line_to_instruction(line);
        instructions.push(instruction)
    }

    let mut cycle: i32 = 0;
    let mut registers: HashMap<char, i32> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Add(register, value) => {
                cycle += 1;
                if cycles.contains(&cycle) {
                    values.push(registers.get(&'x').unwrap().clone() * cycle);
                }
                cycle += 1;
                if cycles.contains(&cycle) {
                    values.push(registers.get(&'x').unwrap().clone() * cycle);
                }
                registers.entry(register).or_insert(1).add_assign(value);
            }
            Instruction::Noop => {
                cycle += 1;
                if cycles.contains(&cycle) {
                    values.push(registers.get(&'x').unwrap().clone() * cycle);
                }
            }
        }
    }

    println!("Part One: {}", values.iter().sum::<i32>())
}

#[derive(Debug)]
enum Instruction {
    Add(char, i32),
    Noop,
}

fn line_to_instruction(line: String) -> Instruction {
    if &line[0..3] == "add" {
        Instruction::Add(
            line.chars().nth(3).unwrap_or_default(),
            line[5..].parse().unwrap_or_default(),
        )
    } else {
        Instruction::Noop
    }
}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
