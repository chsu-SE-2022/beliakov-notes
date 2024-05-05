use std::str::Lines;
use rand::{random, Rng};

#[derive(Debug, Clone)]
pub struct Graph {
    pub vertex_count: usize,
    pub adjacency: Vec<Vec<usize>>,
}

impl Graph {
    /// Initialize the graph
    pub fn new(size: usize) -> Self {
        Self {
            vertex_count: size,
            adjacency: vec![vec![]; size]
        }
    }
    /// Add edge between lhs and rhs
    pub fn add_edge(&mut self, lhs: usize, rhs: usize) {
        self.adjacency[lhs].push(rhs);
        self.adjacency[rhs].push(lhs);
    }
    /// Remove edge between lhs and rhs
    pub fn remove_edge(&mut self, lhs: usize, rhs: usize) {
        let rhs_index = self.adjacency[rhs].iter().position(|x| *x == lhs);
        let lhs_index = self.adjacency[lhs].iter().position(|x| *x == rhs);

        if rhs_index.is_some() && lhs_index.is_some() {
            self.adjacency[rhs].remove(rhs_index.unwrap().clone());
            self.adjacency[lhs].remove(lhs_index.unwrap().clone());
        }

    }
    /// Check if the graph is an eulerian cycle
    pub fn is_eulerian(&self) -> bool {
        let connected = self.check_connected();
        let even = self.adjacency.iter()
            .all(|f|
                f.len() != 0 && f.len() % 2 != 1);
        connected && even
    }

    pub fn check_connected(&self) -> bool {
        self.adjacency.iter().all(|f| f.iter().all(|idx| {
            let mut visited = vec![false; self.vertex_count];
            self.reachable_vertices(*idx, &mut visited) == self.vertex_count
        }))
    }
    /// Count reachable vertices via depth-first-search
    fn reachable_vertices(&self, v: usize, visited: &mut Vec<bool>) -> usize {
        let mut count = 1;
        visited[v] = true;
        for i in &self.adjacency[v] {
            if visited[*i] == false {
                count += self.reachable_vertices(*i, visited)
            }
        }
        count
    }
    /// Check if edge lhs-rhs is valid for the cycle
    fn is_valid_next_edge(&mut self, lhs: i32, rhs: i32) -> bool {
        // If there's only one vertex, it's valid
        if self.adjacency[lhs as usize].len() == 1 {
            return true;
        }
        // If there are no adjacent vertices, the path is over
        if self.adjacency[lhs as usize].len() == 0 {
            return false;
        }

        // Count vertices available from lhs
        let mut visited = vec![false; self.vertex_count];
        let count_prior = self.reachable_vertices(lhs as usize, &mut visited);

        self.remove_edge(lhs as usize, rhs as usize);
        // Count vertices available from lhs which don't disconnect the graph
        let mut visited = vec![false; self.vertex_count];
        let count_after = self.reachable_vertices(lhs as usize, &mut visited);
        // Return the edge to the adjacency list
        self.add_edge(lhs as usize, rhs as usize);

        count_prior <= count_after
    }
    pub fn print_euler_tour(&mut self) {
        let mut rng = rand::thread_rng();
        let first = rng.gen_range(0..self.vertex_count - 1);
        self.print_step(first);
    }

    fn print_step(&mut self, u: usize) {
        for vertex in self.adjacency[u].clone().iter() {
            if self.is_valid_next_edge(u as i32, vertex.clone() as i32) {
                // println!("{u}, {:?}", self.adjacency[u]);
                print!("{u} -> {vertex}; ");
                self.remove_edge(u, vertex.clone());
                self.print_step(vertex.clone());
            } else {
                // print!("{vertex}");
                break;
            }
        }
    }
}