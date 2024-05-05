mod structs;
mod egui_graph_prep;

use std::fs;
use std::io::{Read, read_to_string};
use std::str::Lines;
use eframe::run_native;
use egui_graphs::*;
use crate::egui_graph_prep::BasicApp;
use crate::structs::Graph;


fn main() {
    let mut file = fs::read_to_string("input.txt").unwrap();
    let mut file = file.trim().lines();
    let size = file.next().unwrap().parse::<usize>().unwrap();
    let mut nodes = init_nodes(file, size);

    // Вывод матрицы
    for (idx, vertex) in nodes.adjacency
        .iter()
        .enumerate() {
        println!("Вершина {idx} со связями {vertex:?}");
    }
    let cl = nodes.clone();
    // Эйлеров цикл возможен только при чётности всех вершин графа

    draw(cl);

    if nodes.is_eulerian() {
        println!("Eulerian circuit exists:");
        nodes.print_euler_tour();
    } else {
        println!("Eulerian graph doesn't exist");
    }
}

fn draw(cl: Graph) {
    let native_opt = eframe::NativeOptions::default();
    run_native("graphs", native_opt, Box::new(|cc| Box::new(BasicApp::new(cc, cl)))).unwrap();
}

fn init_nodes(mut lines: Lines, size: usize) -> Graph {
    let mut nodes = Graph::new(size);
    for line in lines {
        let (a, b) = line.split_once(' ').unwrap();
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        nodes.add_edge(a, b);
    }
    for mut i in &mut nodes.adjacency {
        i.sort();
    }
    nodes
}