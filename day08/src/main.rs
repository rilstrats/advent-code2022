use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut grid_width: usize = 0;
    let mut grid_height: usize = 0;
    let mut tree_grid: Vec<Tree> = Vec::new();

    // const FILE: &str = "data/practice.txt";
    const FILE: &str = "data/input.txt";

    // read in data and store in struct
    let lines = read_lines(FILE).expect("Didn't find FILE: {FILE}!");
    for line in lines {
        let line = line.expect("Couldn't read line!");
        if grid_width == 0 {
            grid_width = line.len()
        }
        for num in line.chars() {
            tree_grid.push(Tree::new(num as u8));
        }
        grid_height += 1;
    }

    // find tallest trees from east and west

    let mut tallest_from_n: HashMap<usize, u8> = HashMap::new();
    for x in 0..grid_width {
        tallest_from_n.insert(x, 0);
    }

    let mut tallest_from_w: HashMap<usize, u8> = HashMap::new();
    for y in 0..grid_height {
        tallest_from_w.insert(y, 0);
    }

    for x in 0..grid_width {
        for y in 0..grid_height {
            let mut tree = tree_grid
                .get_mut(grid_width * y + x)
                .expect("Didn't find tree_grid: ({x},{y})!");
            if &tree.height
                > tallest_from_n
                    .get(&x)
                    .expect("Didn't find tallest_from_n: {x}!")
            {
                // tree.vis_from_n = true;
                tree.visible = true;
                tallest_from_n.insert(x, tree.height);
            }
            if &tree.height
                > tallest_from_w
                    .get(&y)
                    .expect("Didn't find tallest_from_w: {y}!")
            {
                // tree.vis_from_w = true;
                tree.visible = true;
                tallest_from_w.insert(y, tree.height);
            }
        }
    }

    // find tallest trees from south and east

    let mut tallest_from_s: HashMap<usize, u8> = HashMap::new();
    for x in 0..grid_width {
        tallest_from_s.insert(x, 0);
    }

    let mut tallest_from_e: HashMap<usize, u8> = HashMap::new();
    for y in 0..grid_height {
        tallest_from_e.insert(y, 0);
    }

    for x in (0..grid_width).rev() {
        for y in (0..grid_height).rev() {
            let mut tree = tree_grid
                .get_mut(grid_width * y + x)
                .expect("Didn't find tree_grid: ({x},{y})!");
            if &tree.height
                > tallest_from_s
                    .get(&x)
                    .expect("Didn't find tallest_from_s: {x}!")
            {
                // tree.vis_from_s = true;
                tree.visible = true;
                tallest_from_s.insert(x, tree.height);
            }
            if &tree.height
                > tallest_from_e
                    .get(&y)
                    .expect("Didn't find tallest_from_e: {y}!")
            {
                // tree.vis_from_e = true;
                tree.visible = true;
                tallest_from_e.insert(y, tree.height);
            }
        }
    }

    // display_tree_grid_visibility(&tree_grid, grid_width, grid_height);

    let mut visible_count: usize = 0;

    for tree in &tree_grid {
        if tree.visible == true {
            visible_count += 1;
        }
    }

    println!("Part One: {}", visible_count);

    for x in 0..grid_width {
        for y in 0..grid_height {
            let tree_height = tree_grid
                .get(grid_width * y + x)
                .expect("Didn't find tree_grid: ({x},{y})!")
                .height;

            // north
            let mut see_n = 0;
            for y_n in (0..y).rev() {
                see_n += 1;
                let tree_n_height = tree_grid
                    .get(grid_width * y_n + x)
                    .expect("Didn't find tree_grid: ({x},{y})!")
                    .height;
                if tree_height <= tree_n_height {
                    break;
                }
            }
            let mut see_s = 0;

            // south
            for y_s in (y + 1)..grid_height {
                see_s += 1;
                let tree_s_height = tree_grid
                    .get(grid_width * y_s + x)
                    .expect("Didn't find tree_grid: ({x},{y})!")
                    .height;
                if tree_height <= tree_s_height {
                    break;
                }
            }

            // east
            let mut see_e = 0;
            for x_e in (x + 1)..grid_width {
                see_e += 1;
                let tree_e_height = tree_grid
                    .get(grid_width * y + x_e)
                    .expect("Didn't find tree_grid: ({x},{y})!")
                    .height;
                if tree_height <= tree_e_height {
                    break;
                }
            }

            // west
            let mut see_w = 0;
            for x_w in (0..x).rev() {
                see_w += 1;
                let tree_w_height = tree_grid
                    .get(grid_width * y + x_w)
                    .expect("Didn't find tree_grid: ({x},{y})!")
                    .height;
                if tree_height <= tree_w_height {
                    break;
                }
            }

            let mut tree = tree_grid
                .get_mut(grid_width * y + x)
                .expect("Didn't find tree_grid: ({x},{y})!");

            // tree.see_n = see_n;
            // tree.see_e = see_e;
            // tree.see_s = see_s;
            // tree.see_w = see_w;
            tree.scenic_score = see_n as usize * see_e as usize * see_s as usize * see_w as usize;
            // println!("{}", tree.scenic_score)
        }
    }

    let mut best_scenic_score: usize = 0;

    for tree in &tree_grid {
        if tree.scenic_score > best_scenic_score {
            best_scenic_score = tree.scenic_score.clone();
        }
    }

    // display_tree_grid_scenic_score(&tree_grid, grid_width, grid_height);

    println!("Part Two: {}", best_scenic_score)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn display_tree_grid_visibility(tree_grid: &Vec<Tree>, tree_width: usize, tree_height: usize) {
    for x in 0..tree_width {
        for y in 0..tree_height {
            let tree = tree_grid
                .get(tree_width * y + x)
                .expect("Didn't find tree_grid: ({x},{y})!");
            if tree.visible {
                print!("T");
            } else {
                print!("F");
            }
        }
        print!("\n")
    }
}

fn display_tree_grid_scenic_score(tree_grid: &Vec<Tree>, tree_width: usize, tree_height: usize) {
    for x in 0..tree_width {
        for y in 0..tree_height {
            let tree = tree_grid
                .get(tree_width * y + x)
                .expect("Didn't find tree_grid: ({x},{y})!");

            print!("{}", tree.scenic_score);
        }
        print!("\n")
    }
}

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: bool,
    // see_n: u8,
    // see_e: u8,
    // see_s: u8,
    // see_w: u8,
    scenic_score: usize,
    // vis_from_n: bool,
    // vis_from_e: bool,
    // vis_from_s: bool,
    // vis_from_w: bool,
}

impl Tree {
    fn new(height: u8) -> Tree {
        Tree {
            height,
            visible: false,
            // see_n: 0,
            // see_e: 0,
            // see_s: 0,
            // see_w: 0,
            scenic_score: 0,
            // vis_from_n: false,
            // vis_from_e: false,
            // vis_from_s: false,
            // vis_from_w: false,
        }
    }
}
