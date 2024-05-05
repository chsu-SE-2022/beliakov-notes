use std::fs::File;
use std::io::{BufRead, BufReader};

mod structs;
use structs::*;

fn main() {
    io_loop();
}

fn io_loop() {
    let path = "./warnpeace.txt".to_owned();
    let tree = read_file(path);
    loop {
        println!("Выберите действие\n\
    1. Вывести слова в алфавитном порядке\n\
    2. Вывести слова в порядке частоты их появления в тексте\n\
    3. Вывести слова определённой длины\n\
    4. Найти слово в тексте");
        let mut act = String::new();
        std::io::stdin().read_line(&mut act).unwrap();
        let action = act.trim().parse();
        match action {
            Ok(1) => {
                let mut file = File::create("alphabetical.txt").unwrap();
                tree.get_list_alphabetical(&mut file);
                println!("Создан файл alphabetical.txt");
                continue;
            },
            Ok(2) => {
                let mut file = File::create("frequency.txt").unwrap();
                tree.get_list_frequency(&mut file);
                println!("Создан файл frequency.txt");
                continue;
            },
            Ok(3) => {
                println!("Введите максимальный размер:");
                let mut size = String::new();
                std::io::stdin().read_line(&mut size).unwrap();

                let size_parsed = size.trim().parse::<usize>();
                match size_parsed {
                    Ok(s) => {
                        let mut file = File::create("filtered.txt").unwrap();
                        tree.get_list_filtered(&mut file, s);
                    }
                    Err(e) => println!("Error: {e}"),
                }
                println!("Создан файл filtered.txt");
                continue;
            },
            Ok(4) => {
                println!("Введите слово:");
                let mut word = String::new();
                std::io::stdin().read_line(&mut word).unwrap();
                let result = tree.find(word.trim().to_owned());
                match result {
                    Some(word) => {
                        println!("{} - {}", word.word, word.frequency);
                    }
                    None => {
                        println!("Слово не найдено")
                    },
                };
                continue;
            },
            Err(ref e) => {
                println!("{e}");
                println!("Некорректный ввод, введите цифру от 1 до 4");
                continue;
            }
            _ => {
                println!("Некорректный ввод, введите цифру от 1 до 4");
                continue;
            }
        }
    }
}
fn read_file(path: String) -> Tree {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut tree = Tree::new();
    for i in reader.lines() {
        let split = i
            .unwrap()
            .split_whitespace()
            .map(|x| x.trim_matches(|x| !char::is_alphabetic(x)))
            .filter(|x| !x.is_empty())
            .map(String::from)
            .collect();
        values_from_words(split, &mut tree);
    }
    tree
}
