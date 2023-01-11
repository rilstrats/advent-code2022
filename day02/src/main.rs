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

        let mut score1 = 0;
        let mut score2 = 0;

        enum Choice {
            Rock,
            Paper,
            Scissors,
        }

        fn choice_points(choice: Choice) -> i32 {
            match choice {
                Choice::Rock => 1,
                Choice::Paper => 2,
                Choice::Scissors => 3,
            }
        }

        enum Outcome {
            Lose,
            Draw,
            Win,
        }

        fn outcome_points(response: Outcome) -> i32 {
            match response {
                Outcome::Lose => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
            }
        }

        for line in lines {
            if let Ok(line) = line {
                let opponent: Choice = match line.chars().next().expect("Didn't find a char!") {
                    'A' => Choice::Rock,
                    'B' => Choice::Paper,
                    'C' => Choice::Scissors,
                    _ => continue,
                };

                let choice1: Choice = match line.chars().last().expect("Didn't find a char!") {
                    'X' => Choice::Rock,
                    'Y' => Choice::Paper,
                    'Z' => Choice::Scissors,
                    _ => continue,
                };

                let outcome1 = match opponent {
                    Choice::Rock => match choice1 {
                        Choice::Rock => Outcome::Draw,
                        Choice::Paper => Outcome::Win,
                        Choice::Scissors => Outcome::Lose,
                    },
                    Choice::Paper => match choice1 {
                        Choice::Rock => Outcome::Lose,
                        Choice::Paper => Outcome::Draw,
                        Choice::Scissors => Outcome::Win,
                    },
                    Choice::Scissors => match choice1 {
                        Choice::Rock => Outcome::Win,
                        Choice::Paper => Outcome::Lose,
                        Choice::Scissors => Outcome::Draw,
                    },
                };

                let choice1 = choice_points(choice1);
                let outcome1 = outcome_points(outcome1);

                score1 += choice1 + outcome1;

                let outcome2: Outcome = match line.chars().last().expect("Didn't find a char!") {
                    'X' => Outcome::Lose,
                    'Y' => Outcome::Draw,
                    'Z' => Outcome::Win,
                    _ => continue,
                };

                let choice2: Choice = match opponent {
                    Choice::Rock => match outcome2 {
                        Outcome::Draw => Choice::Rock,
                        Outcome::Win => Choice::Paper,
                        Outcome::Lose => Choice::Scissors,
                    },
                    Choice::Paper => match outcome2 {
                        Outcome::Lose => Choice::Rock,
                        Outcome::Draw => Choice::Paper,
                        Outcome::Win => Choice::Scissors,
                    },
                    Choice::Scissors => match outcome2 {
                        Outcome::Win => Choice::Rock,
                        Outcome::Lose => Choice::Paper,
                        Outcome::Draw => Choice::Scissors,
                    },
                };

                let choice2 = choice_points(choice2);
                let outcome2 = outcome_points(outcome2);

                score2 += choice2 + outcome2
            }
        }

        println!("Part One: {score1}");
        println!("Part Two: {score2}");

        // for total in totals {
        //     println!("{total}")
        // }
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
