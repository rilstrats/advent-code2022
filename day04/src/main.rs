use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        // println!("{:?}", lines);
        let mut total_overlap = 0;
        let mut any_overlap = 0;
        for line in lines {
            if let Ok(partners) = line {
                // println!("{partners}");
                let (partner1, partner2) = partners.split_once(',').unwrap_or_default();
                // println!("{partner1},{partner2}");
                let (partner1_start, partner1_end) = partner1.split_once('-').unwrap_or_default();
                let (partner2_start, partner2_end) = partner2.split_once('-').unwrap_or_default();
                // println!("{partner1_start}-{partner1_end},{partner2_start}-{partner2_end}");
                let partner1_start: i32 = partner1_start.parse().unwrap_or_default();
                let partner1_end: i32 = partner1_end.parse().unwrap_or_default();
                let partner2_start: i32 = partner2_start.parse().unwrap_or_default();
                let partner2_end: i32 = partner2_end.parse().unwrap_or_default();
                // println!("{partner1_start}-{partner1_end},{partner2_start}-{partner2_end}");

                if (partner1_start <= partner2_start && partner1_end >= partner2_end)
                    || (partner1_start >= partner2_start && partner1_end <= partner2_end)
                {
                    total_overlap += 1
                }
                if (partner1_start <= partner2_start && partner1_end >= partner2_start)
                    || (partner1_start <= partner2_end && partner1_end >= partner2_end)
                    || (partner2_start <= partner1_start && partner2_end >= partner1_start)
                    || (partner2_start <= partner1_end && partner2_end >= partner1_end)
                {
                    any_overlap += 1
                }
            }
        }
        println!("Part One: {}", total_overlap);
        println!("Part Two: {}", any_overlap);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
