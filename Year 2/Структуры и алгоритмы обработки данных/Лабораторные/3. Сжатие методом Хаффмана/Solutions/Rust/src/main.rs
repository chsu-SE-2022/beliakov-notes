mod compress;
mod decompress;
mod structs;
// mod r#pisha

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Add;
use crate::compress::compress;
use crate::decompress::decompress;

fn main() {
    // read file
    // TODO: read a file based on path
    println!("Insert a file path (absolute, sorry)");
    let mut path = "".to_string();
    let input = io::stdin().read_line(&mut path);
    let mut file = File::open("files/file").unwrap();

    compress(&mut file);
    let tmp = fs::read_to_string("files/archived_file").unwrap();
    println!("{:?}", tmp);
    let str: Vec<_> = tmp.split("\n\n").collect();
    for i in str {
        for j in i.as_bytes() {
            print!("{:b} ", j);
        }
    }
    let mut output_file = File::open("files/archived_file").unwrap();
    decompress(&mut output_file);
}

