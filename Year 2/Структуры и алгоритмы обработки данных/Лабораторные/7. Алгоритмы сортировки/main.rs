#![feature(test)]
extern crate test;

use rand::*;
enum Sort {
    Insertion,
    Shaker,
    Shell
}
fn main() {
    let mut rng = thread_rng();
    let vec: Box<[i32]> = (0..20).map(|_| rng.gen_range(0..20)).collect();
    shell_sort(vec);
}
fn insertion_sort(mut vec: Box<[i32]>) {
    for i in 1..vec.len() {
        for j in (0..i).rev() {
            if vec[j] >= vec[j + 1] {
                vec.swap(j, j + 1)
            } else {
                break;
            }
        }

    }
    println!("{:?}", vec)
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
    println!("{:?}", vec)
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
    println!("{:?}", vec);
}
#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use crate::{insertion_sort, shaker_sort, shell_sort};
    use test::Bencher;

    #[bench]
    fn bench_insertion_sort(b: &mut Bencher) {
        let mut rng = thread_rng();
        let vec: Box<[i32]> = (0..10000).map(|_| rng.gen_range(0..20)).collect();
        b.iter(|| insertion_sort(vec.clone()));
    }
    #[bench]
    fn bench_shaker_sort(b: &mut Bencher) {
        let mut rng = thread_rng();
        let vec: Box<[i32]> = (0..10000).map(|_| rng.gen_range(0..20)).collect();
        b.iter(|| shaker_sort(vec.clone()));
    }
    #[bench]
    fn bench_shell_sort(b: &mut Bencher) {
        let mut rng = thread_rng();
        let vec: Box<[i32]> = (0..10000).map(|_| rng.gen_range(0..20)).collect();
        b.iter(|| shell_sort(vec.clone()))
    }
}