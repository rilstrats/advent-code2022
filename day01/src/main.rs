// use std::env;
// use std::fs;

// fn main() {
//     let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("data/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut totals = vec![];
        let mut total = 0;

        for line in lines {
            if let Ok(next) = line {
                if next == "" {
                    totals.push(total);
                    total = 0;
                } else {
                    let next: i32 = next.trim().parse().expect("Not a number!");
                    total += next;
                }
            }
        }

        // for total in totals {
        //     println!("{total}")
        // }

        let max = totals.iter().max().expect("Didn't find a max!");
        println!("Part One: {max}");

        let mut top3 = 0;
        for _ in 0..3 {
            let max = totals.iter().max().expect("Didn't find a max!");
            // println!("{max}");
            top3 += max;
            let position: usize = totals
                .iter()
                .position(|&x| x == *max)
                .expect("Didn't find value!");
            totals.remove(position);
        }
        println!("Part Two: {top3}")
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
