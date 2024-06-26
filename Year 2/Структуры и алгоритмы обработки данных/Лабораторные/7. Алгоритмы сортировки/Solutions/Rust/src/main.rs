#![feature(test)]
extern crate test;

use rand::*;
use std::io::stdin;
use std::time;
use std::time::Duration;

fn main() {
    sort_and_demo();

    let mut io = String::new();
    println!("input an array size for benchmarking");
    let _ = stdin().read_line(&mut io).unwrap();
    let size = io.trim().parse::<u32>().unwrap();
    bench_and_format(size);
}

fn sort_and_demo() {
    let mut rng = thread_rng();
    let vec: Box<[i32]> = (0..5).map(|_| rng.gen_range(-100..100)).collect();
    println!("insertion sort:");
    insertion_sort_formatted(vec.clone());

    println!("shaker sort:");
    shaker_sort_formatted(vec.clone());

    println!("shell sort:");
    shell_sort_formatted(vec.clone());
}
fn bench_and_format(size: u32) {
    let (insertion_random, shaker_random, shell_random) = random_bench(size);
    let (insertion_sorted, shaker_sorted, shell_sorted) = sorted_bench(size);
    let (insertion_reversed, shaker_reversed, shell_reversed) = reversed_bench(size);

    println!("With extra ifs \n");

    let mut table = comfy_table::Table::new();
    table
        .set_header(vec![
            "benchmark/case\n(time in seconds)",
            "random",
            "sorted",
            "reversed",
        ])
        .add_row(vec![
            "insertion",
            &insertion_random.as_secs_f32().to_string(),
            &insertion_sorted.as_secs_f32().to_string(),
            &insertion_reversed.as_secs_f32().to_string(),
        ])
        .add_row(vec![
            "shaker",
            &shaker_random.as_secs_f32().to_string(),
            &shaker_sorted.as_secs_f32().to_string(),
            &shaker_reversed.as_secs_f32().to_string(),
        ])
        .add_row(vec![
            "shell",
            &shell_random.as_secs_f32().to_string(),
            &shell_sorted.as_secs_f32().to_string(),
            &shell_reversed.as_secs_f32().to_string(),
        ]);
    println!("{}", table);

    let (insertion_random, shaker_random, shell_random) = random_bench_fast(size);
    let (insertion_sorted, shaker_sorted, shell_sorted) = sorted_bench_fast(size);
    let (insertion_reversed, shaker_reversed, shell_reversed) = reversed_bench_fast(size);
    println!("\n\nWithout extra ifs");

    let mut table = comfy_table::Table::new();
    table
        .set_header(vec![
            "benchmark/case\n(time in seconds)",
            "random",
            "sorted",
            "reversed",
        ])
        .add_row(vec![
            "insertion",
            &insertion_random.as_secs_f32().to_string(),
            &insertion_sorted.as_secs_f32().to_string(),
            &insertion_reversed.as_secs_f32().to_string(),
        ])
        .add_row(vec![
            "shaker",
            &shaker_random.as_secs_f32().to_string(),
            &shaker_sorted.as_secs_f32().to_string(),
            &shaker_reversed.as_secs_f32().to_string(),
        ])
        .add_row(vec![
            "shell",
            &shell_random.as_secs_f32().to_string(),
            &shell_sorted.as_secs_f32().to_string(),
            &shell_reversed.as_secs_f32().to_string(),
        ]);
    println!("{}", table);
}
fn random_bench(size: u32) -> (Duration, Duration, Duration) {
    let mut rng = thread_rng();

    let random_vec: Box<[i32]> = (0..size).map(|_| rng.gen_range(0..20)).collect();

    let start = time::Instant::now();
    insertion_sort_formatted(random_vec.clone());
    let insertion_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shaker_sort_formatted(random_vec.clone());
    let shaker_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shell_sort_formatted(random_vec.clone());
    let shell_end = time::Instant::now() - start;
    (insertion_end, shaker_end, shell_end)
}
fn sorted_bench(size: u32) -> (Duration, Duration, Duration) {
    let vec: Box<[i32]> = (0..size as i32).collect();

    let start = time::Instant::now();
    insertion_sort_formatted(vec.clone());
    let insertion_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shaker_sort_formatted(vec.clone());
    let shaker_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shell_sort_formatted(vec.clone());
    let shell_end = time::Instant::now() - start;
    (insertion_end, shaker_end, shell_end)
}
fn reversed_bench(size: u32) -> (Duration, Duration, Duration) {
    let vec: Box<[i32]> = (0..size as i32).rev().collect();

    let start = time::Instant::now();
    insertion_sort_formatted(vec.clone());
    let insertion_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shaker_sort_formatted(vec.clone());
    let shaker_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shell_sort_formatted(vec.clone());
    let shell_end = time::Instant::now() - start;
    (insertion_end, shaker_end, shell_end)
}
fn insertion_sort_formatted(mut vec: Box<[i32]>) {
    if vec.len() < 30 {
        println!("{:?}", vec);
    }
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 {
            if vec[j - 1] > vec[j] {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == j - 1 || idx == j {
                            print!("\x1b[1;91m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
                vec.swap(j - 1, j);
            } else {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == j - 1 || idx == j {
                            print!("\x1b[1;94m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
                break;
            }
            j -= 1;
        }
    }
}
fn shaker_sort_formatted(mut vec: Box<[i32]>) {
    if vec.len() < 30 {
        println!("{:?}", vec);
    }
    loop {
        let mut swapped: bool = false;
        for i in 0..vec.len() - 1 {
            if vec[i] > vec[i + 1] {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == i || idx == i + 1 {
                            print!("\x1b[1;91m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
                vec.swap(i, i + 1);

                swapped = true;
            } else {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == i || idx == i + 1 {
                            print!("\x1b[1;94m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
            }
        }
        if !swapped {
            break;
        }
        for i in (0..vec.len() - 1).rev() {
            if vec[i] > vec[i + 1] {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == i || idx == i + 1 {
                            print!("\x1b[1;91m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
                vec.swap(i, i + 1);
                swapped = true;
            } else {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == i || idx == i + 1 {
                            print!("\x1b[1;94m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
            }
        }
        if !swapped {
            break;
        }
    }
}
fn shell_sort_formatted(mut vec: Box<[i32]>) {
    if vec.len() < 30 {
        println!("{vec:?}")
    }
    let mut gap = vec.len() / 2;

    while gap > 0 {
        for i in gap..vec.len() {
            let temp = vec[i];
            let mut j = i;

            if vec[j - gap] <= temp {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == j - gap || idx == j {
                            print!("\x1b[1;94m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
            }

            while j >= gap && vec[j - gap] > temp {
                if vec.len() < 30 && vec.len() != 0 {
                    for (idx, val) in vec.iter().enumerate() {
                        if idx == j - gap || idx == j {
                            print!("\x1b[1;91m{:<4}\x1b[0m", val)
                        } else {
                            print!("{:<4}", val)
                        }
                    }
                    println!()
                }
                vec[j] = vec[j - gap];
                j -= gap;
            }
            vec[j] = temp;
        }
        gap /= 2;
    }
    if vec.len() < 30 {
        println!("{vec:?}")
    }
}
fn random_bench_fast(size: u32) -> (Duration, Duration, Duration) {
    let mut rng = thread_rng();

    let random_vec: Box<[i32]> = (0..size).map(|_| rng.gen_range(0..20)).collect();

    let start = time::Instant::now();
    crate::insertion_sort(random_vec.clone());
    let insertion_end = time::Instant::now() - start;

    let start = time::Instant::now();
    crate::shaker_sort(random_vec.clone());
    let shaker_end = time::Instant::now() - start;

    let start = time::Instant::now();
    crate::shell_sort_formatted(random_vec.clone());
    let shell_end = time::Instant::now() - start;
    (insertion_end, shaker_end, shell_end)
}
fn sorted_bench_fast(size: u32) -> (Duration, Duration, Duration) {
    let vec: Box<[i32]> = (0..size as i32).collect();

    let start = time::Instant::now();
    crate::insertion_sort(vec.clone());
    let insertion_end = time::Instant::now() - start;

    let start = time::Instant::now();
    crate::shaker_sort(vec.clone());
    let shaker_end = time::Instant::now() - start;

    let start = time::Instant::now();
    crate::shell_sort(vec.clone());
    let shell_end = time::Instant::now() - start;
    (insertion_end, shaker_end, shell_end)
}
fn reversed_bench_fast(size: u32) -> (Duration, Duration, Duration) {
    let vec: Box<[i32]> = (0..size as i32).rev().collect();

    let start = time::Instant::now();
    crate::insertion_sort(vec.clone());
    let insertion_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shaker_sort(vec.clone());
    let shaker_end = time::Instant::now() - start;

    let start = time::Instant::now();
    shell_sort(vec.clone());
    let shell_end = time::Instant::now() - start;
    (insertion_end, shaker_end, shell_end)
}
fn insertion_sort(mut vec: Box<[i32]>) {
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
        }
        // j -= 1;
    }
}
fn shaker_sort(mut vec: Box<[i32]>) {
    loop {
        let mut swapped: bool = false;
        for i in 0..vec.len() - 1 {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        for i in (0..vec.len() - 1).rev() {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
fn shell_sort(mut vec: Box<[i32]>) {
    let mut gap = vec.len() / 2;

    while gap > 0 {
        for i in gap..vec.len() {
            let temp = vec[i];
            let mut j = i;

            while j >= gap && vec[j - gap] > temp {
                vec[j] = vec[j - gap];
                j -= gap;
            }
            vec[j] = temp;
        }

        gap /= 2;
    }
}
