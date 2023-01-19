use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(mut lines) = read_lines("data/input.txt") {
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
        diagram.reverse();
        let matcher: &[_] = &['[',']'];
        for line in diagram {
            println!("{}",line.len())
            // let line = line.split(' ');
            // for stack in line {
            //     let stack = stack.trim_matches(matcher);
            //     print!("{}",stack);
            // }
            // println!("");
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
