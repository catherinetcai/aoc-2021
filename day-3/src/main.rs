use std::io::BufRead;
use std::isize;
use std::{fs::File, io::BufReader};

fn main() {
    let filename = std::env::args().nth(1).expect("no filename given");
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Vec<_>>();

    // Turn lines into vectors of vectors of 0s or 1s
    let mut columnized: Vec<Vec<char>> = Vec::with_capacity(lines.len());

    // Get all of the bits into rowsxcols
    for (i, line) in lines.iter().enumerate() {
        // For each line, we want to dump each into a vector
        let col_length = line.as_ref().unwrap().chars().count();
        columnized.push(Vec::with_capacity(col_length));
        // We then add the bit into the line
        for j in 0..col_length {
            // TODO: This will panic
            let push_char = line.as_ref().unwrap().chars().nth(j).unwrap();
            columnized[i].push(push_char);
        }
    }

    // Initialize the transposition - https://stackoverflow.com/questions/65505015/whats-the-best-way-to-switch-columns-and-rows-in-a-2d-array-in-rust
    let transposed: Vec<Vec<char>> = (0..columnized[0].len())
        .map(|i| columnized.iter().map(|c| c[i]).collect())
        .collect();

    let ep = epsilon(&transposed);
    let gam = gamma(&transposed);

    println!(
        "Part 1, epsilon: {}, gamma: {}, power consumption: {}",
        ep,
        gam,
        ep * gam
    );
}

fn epsilon(transposed: &Vec<Vec<char>>) -> i32 {
    let most_common: String = transposed.iter().map(|col| most_common_bit(col)).collect();
    println!("Epsilon: {}", most_common);

    return binary_to_decimal(most_common);
}

fn most_common_bit(col: &Vec<char>) -> char {
    let zeroes = col.iter().filter(|&i| *i == '0').count();
    let ones = col.iter().filter(|&i| *i == '1').count();

    println!(
        "most common bit, zero count: {}, one count: {}",
        zeroes, ones
    );

    if zeroes > ones {
        return '0';
    } else {
        return '1';
    }
}

fn least_common_bit(col: &Vec<char>) -> char {
    let zeroes = col.iter().filter(|&i| *i == '0').count();
    let ones = col.iter().filter(|&i| *i == '1').count();

    println!(
        "least common bit, zero count: {}, one count: {}",
        zeroes, ones
    );
    if zeroes > ones {
        return '1';
    } else {
        return '0';
    }
}

fn gamma(transposed: &Vec<Vec<char>>) -> i32 {
    let least_common: String = transposed.iter().map(|col| least_common_bit(col)).collect();
    println!("Gamma: {}", least_common);

    return binary_to_decimal(least_common);
}

fn binary_to_decimal(bin: String) -> i32 {
    return isize::from_str_radix(&bin.to_string(), 2).unwrap() as i32;
}
