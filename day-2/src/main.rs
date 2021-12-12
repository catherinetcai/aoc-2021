use std::io::BufRead;
use std::{fs::File, io::BufReader};

const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

fn main() {
    let filename = std::env::args().nth(1).expect("no filename give");
    let file = File::open(filename).unwrap();

    let mut reader = BufReader::new(file);
    let lines = reader.lines().collect::<Vec<_>>();
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    let mut depth = 0;

    for (index, line) in lines.iter().enumerate() {
        // Split by direction and number
        let mut split = line.as_ref().unwrap().split(' ').collect::<Vec<&str>>();

        let direction = split[0];
        let steps: i32 = split[1].parse().unwrap();

        match direction {
            FORWARD => {
                horizontal = horizontal + steps;
                depth = depth + (aim * steps);
            }
            DOWN => {
                vertical = vertical + steps;
                aim += steps;
            }
            UP => {
                vertical = vertical - steps;
                aim -= steps;
            }
            _ => {}
        }
    }

    println!("Part one solution is: {}", horizontal * vertical);
    println!("Part two solution is: {}", horizontal * depth);
}
