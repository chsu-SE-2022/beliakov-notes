use std::fs;
use std::fs::File;
use std::io::{self, Write};
fn main() {
    // Reading L and P from input
    let mut rope_length = String::new();
    let mut rope_cut = String::new();
    println!("input length: ");
    io::stdin().read_line(&mut rope_length).expect("Failed to read rope length");
    println!("input rope cut: ");
    io::stdin().read_line(&mut rope_cut).expect("Failed to read rope cut");
    // Parsing input because we need usize lol
    let rope_length: usize = rope_length.trim().parse().expect("Please type a number!");
    let rope_cut: usize = rope_cut.trim().parse().expect("Please type a number!");
    // Creating vector of nail distances
    let mut nails = Vec::new();
    let file = fs::read_to_string("input.txt").unwrap();
    let lines = file.lines();
    for line in lines {
        nails.push(line.trim().parse::<usize>().unwrap());
    }

    let mut combinations = Vec::new();
    // combo count = 2 ^ length count
    for i in 0..(1 << nails.len()) {
        let mut sum = 0;
        let mut combination = Vec::new();
        for (j, nail) in nails.iter().enumerate() {
            // Checking i & (1 << j) helps us get all the combinations
            //  3 & (1 << j) is true for j = 1, 2
            //  8 & (1 << j) is true for j = 8
            // 15 & (1 << j) is true for j = 1, 2, 4, 8
            if i & (1 << j) != 0 {
                sum += nail;
                if combination.contains(&j) {
                    sum += rope_cut;
                }
                else {
                    sum += 2 * rope_cut;
                }
                combination.push(j+1);
            }
        }
        if sum <= rope_length {
            combinations.push(combination);
        }
    }

    let mut output = File::create("output.txt").unwrap();
    for combination in combinations {
        let mut line: String = String::new();
        for i in 0..=nails.len() {
            line += &(i + 1).to_string();
            if combination.contains(&(i + 1)) {
                line.push('-')
            }
            else {
                line.push(' ');
            }
        }
        writeln!(output, "{}", line).expect("Could not write to file");
    }
}