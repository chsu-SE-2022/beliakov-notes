use std::collections::{BTreeMap, HashMap};
use std::fs::{File, read};
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::{fs, i32, io, str, u8};
use std::str::from_utf8;
use crate::compress::BUF_SIZE;
use crate::structs::{build_huffman_code, build_huffman_tree, Node};

pub fn decompress(file: &mut File) {
    let mut frequency_map: BTreeMap<u8, i32> = BTreeMap::new();
    let file = io::read_to_string(file).unwrap();
    let (frequency, code) = file.split_once("\n\n").unwrap();
    read_frequency_map(&mut frequency_map, frequency);
    let tree = build_huffman_tree(&mut frequency_map);
    let codes = build_huffman_code(tree);
    let inverted: HashMap<String, u8> = codes.iter()
        .map(|(k, v)| (v.clone(), *k)).collect();
    println!("codes: {codes:?}");
    write_file(&inverted, code.as_bytes());
}

fn write_file(code_map: &HashMap<String, u8>, encoded_string: &[u8]) {
    let mut binary_string = String::new();
    for i in encoded_string {
        let sub = format!("{i:b}");
        binary_string.push_str(&sub);
    }
    let padding_length = binary_string.len();
    for i in 0..(8 - padding_length) {
        binary_string.push_str("0");
    }

    let mut file = File::create("files/unarchived_file").unwrap();
    loop {
        for (k, v) in code_map {
            if binary_string.starts_with(k) {
                write!(file, "{v}").expect("TODO: panic message");
                println!("{v}");
                for i in 0..k.chars().count() {
                    binary_string.remove(0);
                }
                continue;
            }
            else {
                break;
            }
        }
    }

}

fn read_frequency_map(frequency_map: &mut BTreeMap<u8, i32>, frequency: &str) {
    let freqs: Vec<&str> = frequency.split(";").collect();
    for i in freqs {
        let (byte, size) = i.split_at(1);
        let size = i32::from_str_radix(size, 10).unwrap();
        let byte = byte.as_bytes()[0];
        frequency_map.insert(byte, size);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::decompress::fill_frequency_map;

    #[test]
    fn test_frequency_parsing() {
        let mut correct_map: BTreeMap<u8, i32> = BTreeMap::new();
        correct_map.insert('A' as u8, 6);
        correct_map.insert('B' as u8, 1);
        correct_map.insert('C' as u8, 6);
        correct_map.insert('D' as u8, 2);
        correct_map.insert('E' as u8, 5);

        let frequencies = "A6;B1;C6;D2;E5;";

        let mut test_map: BTreeMap<u8, i32> = BTreeMap::new();

        fill_frequency_map(frequencies, &mut test_map);

        assert_eq!(correct_map, test_map);
    }
    #[test]
    fn test_byte_parsing() {
        let mut correct_map: BTreeMap<u8, i32> = BTreeMap::new();
        correct_map.insert('A' as u8, 6);
        correct_map.insert('B' as u8, 1);
        correct_map.insert('C' as u8, 6);
        correct_map.insert('D' as u8, 2);
        correct_map.insert('E' as u8, 5);

        let bytes = "ª¡ÿäª ".as_bytes();
    }
}