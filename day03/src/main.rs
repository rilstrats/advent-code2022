use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(rucksacks) = read_lines("data/input.txt") {
        let mut duplicates: Vec<char> = Vec::new();
        let mut group: Vec<String> = Vec::new();
        let mut badges: Vec<char> = Vec::new();

        // Loop through rucksack file
        for rucksack in rucksacks {
            if let Ok(rucksack) = rucksack {
                // Find compartment divider
                let divider = rucksack.chars().count() / 2;

                // Define comparments
                let (compartment1, compartment2) = rucksack.split_at(divider);
                let compartment1 = compartment1.to_owned();
                let mut compartment2 = compartment2.to_owned();
                // println!("{compartment1} and {compartment2}");

                // Loop through compartment1 to find duplicates in compartment2
                for snack in compartment1.chars() {
                    if compartment2.contains(snack) {
                        duplicates.push(snack);
                        compartment2 = compartment2.replace(snack, "");
                    }
                }

                // Rucksack is part of a group
                group.push(rucksack.clone());

                // If  group length is 3, group is complete
                // As such, start finding badges
                if group.len() == 3 {
                    let mut badge: char = '_';

                    // Find common badge
                    for snack in group[0].chars() {
                        if group[1].contains(snack) && group[2].contains(snack) {
                            badge = snack;
                            badges.push(snack);
                            break;
                        }
                    }

                    // Debug
                    for rucksack in group.iter() {
                        println!("{rucksack}");
                    }
                    println!("{badge}");

                    // Reset group
                    group.clear();
                }
            }
        }

        // println!("{:?}", duplicates);
        let duplicates = letters_to_priority(duplicates);
        let badges = letters_to_priority(badges);

        println!("Part One: {duplicates}");
        println!("Part Two: {badges}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn letters_to_priority(letters: Vec<char>) -> i32 {
    let mut priority = 0;
    for letter in letters {
        priority += letter_to_priority(letter).expect("Character doesn't have priority value!");
    }
    priority
}

fn letter_to_priority(letter: char) -> Result<i32, String> {
    match letter {
        'a' => Ok(1),
        'b' => Ok(2),
        'c' => Ok(3),
        'd' => Ok(4),
        'e' => Ok(5),
        'f' => Ok(6),
        'g' => Ok(7),
        'h' => Ok(8),
        'i' => Ok(9),
        'j' => Ok(10),
        'k' => Ok(11),
        'l' => Ok(12),
        'm' => Ok(13),
        'n' => Ok(14),
        'o' => Ok(15),
        'p' => Ok(16),
        'q' => Ok(17),
        'r' => Ok(18),
        's' => Ok(19),
        't' => Ok(20),
        'u' => Ok(21),
        'v' => Ok(22),
        'w' => Ok(23),
        'x' => Ok(24),
        'y' => Ok(25),
        'z' => Ok(26),
        'A' => Ok(27),
        'B' => Ok(28),
        'C' => Ok(29),
        'D' => Ok(30),
        'E' => Ok(31),
        'F' => Ok(32),
        'G' => Ok(33),
        'H' => Ok(34),
        'I' => Ok(35),
        'J' => Ok(36),
        'K' => Ok(37),
        'L' => Ok(38),
        'M' => Ok(39),
        'N' => Ok(40),
        'O' => Ok(41),
        'P' => Ok(42),
        'Q' => Ok(43),
        'R' => Ok(44),
        'S' => Ok(45),
        'T' => Ok(46),
        'U' => Ok(47),
        'V' => Ok(48),
        'W' => Ok(49),
        'X' => Ok(50),
        'Y' => Ok(51),
        'Z' => Ok(52),
        _ => Err("Character doesn't have priority value!".to_owned()),
    }
}
