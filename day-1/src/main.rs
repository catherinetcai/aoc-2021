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

    let mut rolling_sum_counter = 0;
    let mut prev_rolling_sum = 0;

    for (index, line) in lines.iter().enumerate() {
        if index == 0 {
            prev_rolling_sum = line.as_ref().unwrap().parse::<i32>().unwrap()
                + lines[index + 1].as_ref().unwrap().parse::<i32>().unwrap()
                + lines[index + 2].as_ref().unwrap().parse::<i32>().unwrap();
            continue;
        }

        // Break if we cannot handle another rolling sum
        if index + 3 > lines.len() {
            break;
        }

        let current_sum = line.as_ref().unwrap().parse::<i32>().unwrap()
            + lines[index + 1].as_ref().unwrap().parse::<i32>().unwrap()
            + lines[index + 2].as_ref().unwrap().parse::<i32>().unwrap();

        println!(
            "Previous rolling sum: {}, current sum: {}",
            prev_rolling_sum, current_sum
        );
        if prev_rolling_sum < current_sum {
            rolling_sum_counter += 1;
        }

        prev_rolling_sum = current_sum;
    }

    println!("Part 1 number increased: {}", counter);
    println!("Part 2 number increased: {}", rolling_sum_counter);
}
