use std::collections::HashSet;
use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // const INPUT: &str = "data/example2.txt";
    const INPUT: &str = "data/input.txt";
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let lines = read_lines(INPUT).expect("didn't find file");
    for line in lines {
        let line = line.expect("line not found");
        let direction: &str = line.split_whitespace().next().expect("direction not found");
        let distance: i32 = line
            .split_whitespace()
            .last()
            .expect("distance not found")
            .parse()
            .expect("distance couldn't covert to i32");

        for _ in 0..distance {
            match direction {
                "R" => head.move_east(),
                "L" => head.move_west(),
                "U" => head.move_north(),
                "D" => head.move_south(),
                _ => continue,
            }
            tail.follow(&head);
            // println!("{:?}", head);
            // println!("{:?}", tail);
        }
    }
    // println!("{:#?}", tail.visited);
    println!("Part One: {}", tail.visited.len());

    const KNOTS: i32 = 10;

    let mut knots: Vec<Point> = vec![];
    for _ in 0..KNOTS {
        knots.push(Point::new(0, 0))
    }
    let lines = read_lines(INPUT).expect("didn't find file");

    for line in lines {
        let line = line.expect("line not found");
        let direction: &str = line.split_whitespace().next().expect("direction not found");
        let distance: i32 = line
            .split_whitespace()
            .last()
            .expect("distance not found")
            .parse()
            .expect("distance couldn't covert to i32");

        for _ in 0..distance {
            let head = knots.first_mut().expect("didn't find head");
            match direction {
                "R" => head.move_east(),
                "L" => head.move_west(),
                "U" => head.move_north(),
                "D" => head.move_south(),
                _ => continue,
            }

            for i in 1..KNOTS {
                let leader = knots
                    .get((i - 1) as usize)
                    .expect("didn't find leader")
                    .clone();
                let follower = knots.get_mut(i as usize).expect("didn't find follower");
                follower.follow(&leader);
            }
        }
    }
    for knot in &knots {
        println!("{:#?}", knot);
    }
    println!(
        "Part Two: {}",
        knots.last().expect("didn't find last").visited.len()
    );
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    visited: HashSet<(i32, i32)>,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn follow(&mut self, point: &Point) {
        let dif_ver = point.y - self.y;
        let dif_hor = point.x - self.x;
        let dif_tot = dif_ver.abs() + dif_hor.abs();

        // if same row and not adjacent
        if dif_ver == 0 && dif_tot == 2 {
            if dif_hor == 2 {
                self.move_east();
            } else if dif_hor == -2 {
                self.move_west();
            }
        }
        // if same column
        else if dif_hor == 0 && dif_tot == 2 {
            if dif_ver == 2 {
                self.move_north();
            } else if dif_ver == -2 {
                self.move_south();
            }
        }
        // if diagonal and not adjacent
        else if dif_tot >= 3 {
            // if north
            if dif_ver > 0 {
                // and east
                if dif_hor > 0 {
                    self.move_northeast()
                }
                // and west
                if dif_hor < 0 {
                    self.move_northwest()
                }
            }
            // if south
            else if dif_ver < 0 {
                // and east
                if dif_hor > 0 {
                    self.move_southeast()
                }
                // and west
                if dif_hor < 0 {
                    self.move_southwest()
                }
            }
        }
    }

    fn move_north(&mut self) {
        self.y += 1;
        self.visited.insert((self.x, self.y));
    }

    fn move_northeast(&mut self) {
        self.y += 1;
        self.x += 1;
        self.visited.insert((self.x, self.y));
    }

    fn move_northwest(&mut self) {
        self.y += 1;
        self.x -= 1;
        self.visited.insert((self.x, self.y));
    }

    fn move_south(&mut self) {
        self.y -= 1;
        self.visited.insert((self.x, self.y));
    }

    fn move_southeast(&mut self) {
        self.y -= 1;
        self.x += 1;
        self.visited.insert((self.x, self.y));
    }

    fn move_southwest(&mut self) {
        self.y -= 1;
        self.x -= 1;
        self.visited.insert((self.x, self.y));
    }

    fn move_east(&mut self) {
        self.x += 1;
        self.visited.insert((self.x, self.y));
    }

    fn move_west(&mut self) {
        self.x -= 1;
        self.visited.insert((self.x, self.y));
    }
}

// enum Move {
//     Right(i32),
//     Left(i32),
//     Up(i32),
//     Down(i32),
//     Err(),
// }

// impl Move {
//     fn new(direction: char, distance: i32) -> Result<Self, Error> {
//         match direction {
//             'R' => Ok(Move::Right(distance)),
//             'L' => Ok(Move::Left(distance)),
//             'U' => Ok(Move::Up(distance)),
//             'D' => Ok(Move::Down(distance)),
//             _ => Err(Error),
//         }
//     }
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
