use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::{error, fs, io, u8};
use crate::structs::{build_huffman_code, build_huffman_tree, Node};

pub fn compress(file: &mut File) {
    // Create a base for the frequency map
    let mut map: BTreeMap<u8, i32> = BTreeMap::new();
    // Take a mutable reference because memory usage
    let map_ref = &mut map;
    match parse_file(file, map_ref) {
        Ok(()) => println!("Hashmap build successfully"),
        Err(e) => eprint!("{}", e),
    }
    // Create a Huffman tree
    let tree: Box<Node> = build_huffman_tree(map_ref);
    let codes: HashMap<u8, String> = build_huffman_code(tree);
    write_file(&codes, map_ref, file);
}

fn write_file(codes: &HashMap<u8, String>, frequency: &BTreeMap<u8, i32>, input_file: &mut File) {
    let mut output_file = File::create("files/archived_file").unwrap();
    let mut output_writer = BufWriter::new(&mut output_file);
    let mut input_reader = BufReader::with_capacity(BUF_SIZE, input_file);

    // Write as base-10
    for (byte, freq) in frequency {
        let byte = *byte as char;
        write!(output_writer, "{byte}{freq};").expect("write successful");
    }
    write!(output_writer, "\n\n").expect("separator written successfully");
    let mut code_string: String = String::new();
    // Create a string to fill with codes
    let mut codes_part = String::new();
    loop {
        // Create a buffer of 64 bytes
        let buffer = input_reader.fill_buf().unwrap();
        // Check buffer length as a single borrow
        let buffer_length = buffer.len();

        // If buffer is not empty, fill the string, otherwise append padding
        if buffer_length == 0 {
            let padding_length = 8 - codes_part.as_bytes().len() % 8;
            for _ in 0..padding_length {
                codes_part.push('0');
            }
            if codes_part.len() == 8 {
                println!("padded codes_part: {codes_part}");
                let char_form = u8::from_str_radix(&codes_part, 2).unwrap();
                // println!("{}", char_form);
                write!(output_writer, "{}", char_form as char).expect("char written correctly");
                codes_part.clear();
            }
            input_reader.consume(BUF_SIZE);
            break;
        }

        for byte in buffer {
            let code = codes.get(byte).unwrap();

            code_string.push_str(code);
        }

        let mut code_string_len = code_string.as_bytes().len();

        while code_string_len >= 8 {
            for _ in 0..8.max(code_string.as_bytes().len()) {
                let c = code_string.remove(0);
                // println!("char pushed: {}", c);
                codes_part.push(c);
                // println!("codes_part: {codes_part}");
                if codes_part.len() == 8 {
                    println!("codes_part: {codes_part}");

                    let char_form = u8::from_str_radix(&codes_part, 2).unwrap();
                    // println!("{}", char_form);
                    write!(output_writer, "{}", char_form as char).expect("char written correctly");
                    codes_part.clear();
                }
            }
            code_string_len = code_string.len();
        }
        input_reader.consume(BUF_SIZE);

    }
    input_reader.seek(SeekFrom::Start(0)).unwrap();

    output_writer.flush().expect("flush successful");
}

pub(crate) const BUF_SIZE: usize = 64;
fn parse_file(file: &mut File, map: &mut BTreeMap<u8, i32>) -> io::Result<()> {
    // File BufReader
    let mut bufreader = BufReader::with_capacity(BUF_SIZE, file);
    let reader_ref = &mut bufreader;

    loop {
        // Fill buffer with bytes
        let buffer = reader_ref.fill_buf()?;

        let buffer_length = buffer.len();
        // Exit if no bytes are found => EOF
        if buffer_length == 0 {
            break;
        }
        // Insert byte into hashmap
        fill_hashmap(buffer, map);
        // Shift bufreader
        reader_ref.consume(buffer_length);
    }
    // Return reader to the start for the next read
    reader_ref.seek(SeekFrom::Start(0)).unwrap();
    Ok(())
}
/// Insert all bytes from a buffer into a [char -> frequency BTreeMap](`BTreeMap`)
fn fill_hashmap(bytes: &[u8], btree_map: &mut BTreeMap<u8, i32>) {
    for byte in bytes {
        btree_map.entry(*byte).and_modify(|value| *value += 1).or_insert(1);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::compress::fill_hashmap;

    /// Frequencies are calculated correctly
    #[test]
    fn test_frequency_map() {
        let mut correct_map: BTreeMap<u8, i32> = BTreeMap::new();
        correct_map.insert('A' as u8, 6);
        correct_map.insert('B' as u8, 1);
        correct_map.insert('C' as u8, 6);
        correct_map.insert('D' as u8, 2);
        correct_map.insert('E' as u8, 5);

        let mut test_map: BTreeMap<u8, i32> = BTreeMap::new();
        let input_data = "AAAAAABCCCCCCDDEEEEE".as_bytes();
        fill_hashmap(input_data, &mut test_map);

        assert_eq!(correct_map, test_map);
    }

}
