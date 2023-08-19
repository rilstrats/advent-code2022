use std::{
    env,
    fs::File,
    io::{self, BufRead},
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

    let mut monkeys: Vec<Monkey> = vec![];
    let mut throws: Vec<Vec<u64>> = vec![];

    let lines = read_lines(file_path).expect("Error reading file");
    let mut iter = lines.into_iter();
    loop {
        let Some(Ok(id)) = iter.next() else {break};
        let id: usize = id
            .trim()
            .replace(':', "")
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let mut items = iter.next().unwrap().unwrap();
        let split_point = items.find(':').unwrap();
        items.replace_range(0..=split_point, "");
        let items: Vec<u64> = items
            .trim()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let operation = iter.next().unwrap().unwrap();
        let operation = operation.trim().split(' ').collect::<Vec<&str>>();
        let operation: Operation = match (
            operation.get(4).unwrap().to_owned(),
            operation.get(5).unwrap().to_owned(),
        ) {
            ("+", num) => Operation::Add(num.parse().unwrap()),
            ("*", "old") => Operation::Square,
            ("*", num) => Operation::Multiply(num.parse().unwrap()),
            (_, _) => unreachable!(),
        };

        let test_num: u64 = iter
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .replace(':', "")
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let true_monkey: usize = iter
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .replace(':', "")
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let false_monkey: usize = iter
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .replace(':', "")
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        iter.next();

        monkeys.push(Monkey {
            id,
            items,
            operation,
            test_num,
            true_monkey,
            false_monkey,
            inspect_count: 0,
        });

        throws.push(vec![]);
    }

    // println!("{:#?}", monkeys);

    let mut monkeys2 = monkeys.clone();
    let mut throws2 = throws.clone();

    for _ in 0..20 {
        for monkey in monkeys.iter_mut() {
            monkey.turn(&mut throws)
        }
    }

    let mut first_most: u64 = 0;
    let mut second_most: u64 = 0;

    for monkey in monkeys.iter() {
        println!(
            "Monkey {} inspected {} times",
            monkey.id, monkey.inspect_count
        );
        if monkey.inspect_count > first_most {
            second_most = first_most;
            first_most = monkey.inspect_count;
        } else if first_most > monkey.inspect_count && monkey.inspect_count > second_most {
            second_most = monkey.inspect_count;
        }
    }

    println!("\nPart One: {}\n", first_most * second_most);

    let mut modulus: u64 = 1;

    for monkey in monkeys.iter() {
        modulus = modulus * monkey.test_num
    }

    for _ in 0..10000 {
        for monkey in monkeys2.iter_mut() {
            monkey.turn2(&mut throws2, modulus)
        }
    }

    let mut first_most2: u64 = 0;
    let mut second_most2: u64 = 0;

    for monkey in monkeys2.iter() {
        println!(
            "Monkey {} inspected {} times",
            monkey.id, monkey.inspect_count
        );
        if monkey.inspect_count > first_most2 {
            second_most2 = first_most2;
            first_most2 = monkey.inspect_count;
        } else if first_most2 > monkey.inspect_count && monkey.inspect_count > second_most2 {
            second_most2 = monkey.inspect_count;
        }
    }

    println!("\nPart Two: {}", first_most2 * second_most2);
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test_num: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u64,
}

impl Monkey {
    fn inspect(&mut self, item: u64) -> u64 {
        self.inspect_count += 1;
        match self.operation {
            Operation::Multiply(num) => item * num,
            Operation::Add(num) => item + num,
            Operation::Square => item * item,
        }
    }

    fn relax(&self, item: u64) -> u64 {
        item / 3
    }

    fn test(&self, item: &u64) -> usize {
        if item % self.test_num == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }

    fn throw(&self, item: u64, monkey: usize, throws: &mut Vec<Vec<u64>>) {
        throws.get_mut(monkey).unwrap().push(item);
    }

    fn get_throws(&mut self, throws: &mut Vec<Vec<u64>>) {
        let throws = throws.get_mut(self.id).unwrap();
        while throws.len() > 0 {
            let item = throws.remove(0);
            self.items.push(item);
        }
    }

    fn turn(&mut self, throws: &mut Vec<Vec<u64>>) {
        self.get_throws(throws);
        while self.items.len() > 0 {
            let item = self.items.remove(0);
            let item = self.inspect(item);
            let item = self.relax(item);
            let monkey = self.test(&item);
            self.throw(item, monkey, throws);
        }
    }

    fn turn2(&mut self, throws: &mut Vec<Vec<u64>>, modulus: u64) {
        self.get_throws(throws);
        while self.items.len() > 0 {
            let item = self.items.remove(0) % modulus;
            let item = self.inspect(item) % modulus;
            let monkey = self.test(&item);
            self.throw(item, monkey, throws);
        }
    }
}

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
