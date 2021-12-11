use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    let filename = std::env::args().nth(1).expect("no filename give");
    let file = File::open(filename).unwrap();

    let mut reader = BufReader::new(file);
    let lines = reader.lines().collect::<Vec<_>>();
    let mut counter = 0;
    let mut prev = -1;

    for (index, line) in lines.iter().enumerate() {
        if index == 0 {
            prev = line.as_ref().unwrap().parse().unwrap();
            continue;
        }

        let mut current: i32 = line.as_ref().unwrap().parse().unwrap();

        if prev < current {
            counter += 1;
        }

        prev = current;
    }

    println!("Number increased: {}", counter);
}
