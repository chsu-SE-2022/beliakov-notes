use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let input: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let counts = input[0].split_once(" ").unwrap();
    let rows: usize = counts.0.parse().unwrap();
    let columns: usize = counts.1.parse().unwrap();
    let base: Vec<Vec<i32>> = input
        .iter().skip(1)
        .map(|x| x.split(" ")
        .map(|x| x.parse::<i32>().unwrap()).collect()).collect();
    let mut sum: Vec<Vec<i32>> = vec![vec![0; columns]; rows];
    let mut direction: Vec<Vec<i32>> = vec![vec![0; columns]; rows];
    for (row, arr) in base.iter().enumerate() {
        for (col, value) in  row.iter().enumerate() {
            sum[row][col] += *value;
        }
    }
}
