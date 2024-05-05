use std::fs;
use clap::*;
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 19)]
    length: i32,
    #[arg(short, long, default_value_t = 1)]
    per: i32,
}
fn main() {
    let args = Args::parse();
    let distances: Vec<i32> = fs::read_to_string("input.txt").unwrap().lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let length = args.length;
    let tie = args.per;

    let mut permutations: Vec<Vec<bool>> = vec![];

    for i in 0..distances.len() {
        let mut current = vec![false; distances.len()];
        let mut iter_length = length;
        for j in 0..distances.len() {
            if iter_length - distances[i] - if current[i] { tie } else { tie * 2 } > 0 {
                iter_length = iter_length - distances[i] - if current[i] { tie } else { tie * 2 };
                current[j] = true;
            }
        }
        permutations.push(current);
    }
    dbg!(permutations);
}
