use std::{fs, usize};

fn main() {
    // Parse initial input into a string
    let input: String = fs::read_to_string("input.txt")
        .unwrap();
    let mut input_matrix = input
        .lines()
        .skip(1)
        .map(|x| x
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    let coords: Option<(&str, &str)> = input.lines().collect::<Vec<&str>>()[0].split_once(' ');
    let (x, y) = coords.unwrap();
    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

    println!("input matrix: ");
    for i in &input_matrix{
        for j in i {
            print!("{:<5}", j);
        }
        println!();
    }
    input_matrix.reverse();

    let mut sum_matrix = fill_sum(x, y, &input_matrix);
    println!("Sum matrix: ");
    let mut s = sum_matrix.clone();
    s.reverse();
    for i in s {
        for j in i {
            print!("{:<5}", j);
        }
        println!();
    }
    // sum_matrix.reverse();
    let path_matrix = fill_path(x, y, &sum_matrix, &input_matrix);
    println!("Path matrix: ");
    for i in &path_matrix {
        for j in i {
            print!("{:<5}", j);
        }
        println!();
    }
}
fn fill_sum(x: usize, y: usize, input: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut sum_matrix = vec![vec![0; x]; y];

    for i in 0..y {
        for j in 0..x {
            let up_over = match i.checked_sub(2) {
                Some(y) => {
                    sum_matrix.get(y).and_then(|x| x.get(j)).and_then(|x| Some(*x))
                },
                None => None
            };
            let up_left = match i.checked_sub(1) {
                Some(y) => {
                    if j + 1 < x {
                        sum_matrix.get(y).and_then(|x| x.get(j + 1)).and_then(|x| Some(*x))
                    } else {
                        None
                    }
                }
                None => None,
            };
            let up_right = match i.checked_sub(1) {
                Some(y) => {
                    match j.checked_sub(1) {
                        Some(x) => {
                            sum_matrix.get(y).and_then(|o| o.get(x)).and_then(|o| Some(*o))
                        },
                        None => None,
                    }
                }
                None => None
            };

            let directions = vec![up_over, up_left, up_right];
            let max = directions.iter().flatten().max().unwrap_or(&0);
            sum_matrix[i][j] = input[i][j] + max;
        }
    }
    sum_matrix
}
fn fill_path(x: usize, y: usize, sum_matrix: &Vec<Vec<i32>>, input: &Vec<Vec<i32>>) -> Vec<Vec<String>> {
    let mut path_matrix = vec![vec!["0".to_string(); x]; y];

    let mut max_top = i32::MIN;
    let mut path_x = x;
    let mut path_y = y - 1;
    for i in (0..x).rev() {
        let curr = sum_matrix[y - 1][i];
        max_top = max_top.max(curr);
        if curr == max_top.max(curr) {
            path_x = i;
        }
    }
    // println!("Start: {}", sum_matrix[path_y][path_x]);
    // println!("x: {path_x}, y: {path_y}");
    path_matrix[path_y][path_x] = input[path_y][path_x].to_string();
    while path_y != 0 {
        let mut down_over_pos = (10, 10);
        let down_over= match path_y.checked_sub(2) {
            Some(y) => {
                down_over_pos = (y, path_x);
                sum_matrix.get(y).and_then(|x| x.get(path_x)).and_then(|x| Some(*x))
            }
            None => None,
        };
        let mut down_right_pos = (10, 10);
        let down_right = match path_y.checked_sub(1) {
            Some(y) => {
                if path_x + 1 < x {
                    down_right_pos = (y, path_x + 1);
                    sum_matrix.get(y).and_then(|x| x.get(path_x + 1)).and_then(|x| Some(*x))
                } else {
                    None
                }
            }
            None => None,
        };
        let mut down_left_pos = (10, 10);
        let down_left = match path_y.checked_sub(1) {
            Some(y) => {
                match path_x.checked_sub(1) {
                    Some(x) => {
                        down_left_pos = (y, path_x - 1);
                        sum_matrix.get(y).and_then(|x| x.get(path_x - 1)).and_then(|x| Some(*x))
                    },
                    None => None,
                }
            }
            None => None,
        };
        let directions = vec![down_over, down_right, down_left];
        // println!("{:?}", directions);
        // println!("down_over: {down_over:?}, down_right: {down_right:?}, down_left: {down_left:?}");
        let max = directions.iter().flatten().max().unwrap_or(&0);
        // println!("{:?}", min);
        if Some(*max) == down_over {
            path_y = down_over_pos.0;
            path_x = down_over_pos.1;
        }
        else if Some(*max) == down_right {
            path_y = down_right_pos.0;
            path_x = down_right_pos.1;
        }
        else if Some(*max) == down_left {
            path_y = down_left_pos.0;
            path_x = down_left_pos.1;
        }
        // path_matrix[path_y][path_x] = sum_matrix[path_y][path_x].to_string();
        path_matrix[path_y][path_x] = input[path_y][path_x].to_string();
    }
    path_matrix.reverse();
    path_matrix
}