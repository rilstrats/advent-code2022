use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut pwd: MyPath = MyPath::new();
    let mut dir_files: HashMap<String, Vec<MyFile>> = Default::default();
    if let Ok(lines) = read_lines("data/input.txt") {
        for line in lines {
            // println!("{:#?}", pwd);
            let line = serialize(line.unwrap().to_owned());
            // println!("{:#?}", line);
            let any = line.handle_line();
            // println!("{:#?}", any);
            match any {
                MyAny::Command(command) => command.handle_command(&mut pwd),
                MyAny::File(file) => {
                    let paths = pwd.get_all_paths();
                    // println!("{:#?}", paths);
                    for path in paths {
                        dir_files
                            .entry(path)
                            .or_insert(Vec::new())
                            .push(file.clone());
                    }
                }
                MyAny::Dir(_) => continue,
            }
        }

        let mut dir_size: HashMap<String, u32> = Default::default();

        for (dir, files) in dir_files {
            let mut sum: u32 = 0;
            for file in files {
                sum += file.size;
            }
            dir_size.insert(dir, sum);
        }

        // println!("{:#?}", dir_size)

        let mut sum1: u32 = 0;
        for (_, size) in &dir_size {
            if size <= &100000 {
                sum1 += size;
            }
        }

        println!("Part One: {:#?}", sum1);

        let total_space: u32 = 70000000; // 70 MB
        let needed_space: u32 = 30000000; // 30 MB
        let target_usage: u32 = total_space - needed_space; // 40 MB
        let current_usage = dir_size.get("/").unwrap().to_owned(); // 46.5 MB

        // let difference: u32 = current_usage - target_usage; // 6.5 MB

        let mut best_dir: String = String::from("/");
        let mut best_size: u32 = current_usage;
        let mut best_new_usage: u32 = 0;
        for (dir, size) in &dir_size {
            let new_usage = current_usage - size;
            if new_usage > target_usage || new_usage < best_new_usage {
                continue;
            }
            best_dir = dir.to_owned();
            best_size = size.to_owned();
            best_new_usage = new_usage;
        }

        println!("Part Two: {} {} {}", best_dir, best_size, best_new_usage);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn serialize(line: String) -> Line {
    match line.chars().next().unwrap() {
        // if first char is $ then return Line::Input(line without $)
        '$' => Line::Input(line[2..].to_owned()),
        _ => Line::Output(line.to_owned()),
    }
}

#[derive(Debug)]
struct MyPath {
    path: String,
    path_vec: Vec<String>,
}

impl MyPath {
    fn new() -> MyPath {
        MyPath {
            path: String::from("/"),
            path_vec: vec![],
        }
    }

    // fn get_path(&self) -> String {
    //     self.path.clone()
    // }

    fn get_all_paths(&self) -> Vec<String> {
        let mut all_paths: Vec<String> = Default::default();
        let mut path_vec = self.path_vec.clone();
        while path_vec.len() > 0 {
            all_paths.push(self.fake_pwd(path_vec.clone()));
            path_vec.pop();
        }
        all_paths.push(String::from("/"));
        all_paths
    }

    // fn update_pwd_vec(&mut self) {
    //     // This function should be called after manually updating self.pwd
    //     self.path_vec = self.path.split('/').map(str::to_string).collect();
    // }

    fn update_pwd(&mut self) {
        // this function should be called after manually updating self.pwd_vec
        self.path = String::from("/");
        for dir in &self.path_vec {
            self.path.push_str(&dir);
            self.path.push('/');
        }
    }

    fn fake_pwd(&self, path_vec: Vec<String>) -> String {
        // this function should be called after manually updating self.pwd_vec
        let mut path = String::from("/");
        for dir in path_vec {
            path.push_str(&dir);
            path.push('/');
        }
        path
    }

    fn cd_root(&mut self) {
        self.path_vec.clear();
        self.update_pwd();
    }

    fn cd_up(&mut self) {
        self.path_vec.pop();
        self.update_pwd();
    }

    fn cd(&mut self, dir: String) {
        self.path_vec.push(dir);
        self.update_pwd();
    }
}

#[derive(Debug)]
enum Line {
    Input(String),
    Output(String),
}

impl Line {
    fn handle_line(&self) -> MyAny {
        match self {
            Line::Input(input) => match &input[0..=1] {
                "cd" => MyAny::Command(Command::Change(input[3..].to_owned())),
                _ => MyAny::Command(Command::List()),
                // "ls" => Any::Command(Command::List()),
                // _ => Any::Command(Command::Err(input.to_string())),
            },
            Line::Output(output) => {
                match output.split_whitespace().next().unwrap() {
                    "dir" => MyAny::Dir(MyDir {
                        name: output.split_whitespace().last().unwrap().to_owned(),
                    }),
                    _ => {
                        let (size, name) = output.split_once(' ').unwrap();
                        // let name = name.to_owned();
                        // let size: u32 = size.parse().unwrap();
                        MyAny::File(MyFile {
                            size: size.parse().unwrap(),
                            name: name.to_owned(),
                        })
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum Command {
    List(),
    Change(String),
    // Err(String),
}

impl Command {
    fn handle_command(&self, pwd: &mut MyPath) {
        match self {
            Command::Change(dir) => match &dir[..] {
                ".." => pwd.cd_up(),
                "/" => pwd.cd_root(),
                _ => pwd.cd(dir.to_owned()),
            },
            Command::List() => (),
        }
    }
}

#[derive(Clone, Debug)]
struct MyFile {
    // dir: String,
    size: u32,
    name: String,
}

#[derive(Debug)]
struct MyDir {
    name: String,
}

#[derive(Debug)]
enum MyAny {
    Command(Command),
    File(MyFile),
    Dir(MyDir),
}
