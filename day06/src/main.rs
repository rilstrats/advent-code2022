use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let stream = read_lines("data/input.txt")
        .unwrap()
        .next()
        .unwrap()
        .unwrap();

    let packet_marker = find_rep(&stream, 0, 4);

    println!("{}", packet_marker);

    let message_marker = find_rep(&stream, packet_marker + 1, 14);

    println!("{}", message_marker)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_rep(stream: &str, start: usize, rep_len: usize) -> usize {
    let mut marker: usize = 0;
    for stream_index in start..stream.len() {
        let slice = &stream[stream_index..stream_index + rep_len];
        // println!("start slice loop: {}", slice);
        let mut duplicity = 0;
        for character in slice.chars() {
            // println!("    start character loop: {}", character);
            for slice_index in 0..slice.len() {
                // println!(
                //     "        comparison character: {}",
                //     slice.chars().nth(slice_index).unwrap()
                // );
                if character == slice.chars().nth(slice_index).unwrap() {
                    duplicity += 1;
                }
                // println!("        current duplicity: {}\n", duplicity);
            }
        }
        // println!("slice duplicity: {}\n", duplicity);
        if duplicity <= rep_len {
            marker = stream_index + rep_len;
            break;
        }
    }
    marker
}
