use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const MAXVTEX:usize = 15;
pub struct Graph {
    vindex:usize,
    adj_matrix:[[i32; MAXVTEX]; MAXVTEX],
    vertex_map: HashMap<String,usize>
}
impl Graph {
    fn new() -> Graph {
        Graph {
            vindex:0,
            adj_matrix:[[0; MAXVTEX]; MAXVTEX],
            vertex_map: HashMap::<String,usize>::new()
        }
    }
    fn add_vertex(&mut self, vtex: &str) {
        if !self.vertex_map.contains_key(&vtex.to_string()) {
            self.vertex_map.insert(vtex.to_string(),self.vindex);
            self.vindex += 1;
        }
    }
    fn get_vertex(&self, vtex: &str) -> usize {
        let isfound = self.vertex_map.get(&vtex.to_string());
        match isfound {
            Some(vindex) => *vindex,
            None => panic!("No vertex found!")
        }
    }
    fn add_edge(&mut self, from: &str, to: &str) {
        let from_index = self.get_vertex(from);
        let to_index = self.get_vertex(to);
        let mut from_larger = 1;
        let mut to_larger = 1;
        if from == from.clone().to_lowercase() {
            from_larger = -1;
        }
        if to == to.clone().to_lowercase() {
            to_larger = -1;
        }
        if to == "start" || from == "end" {
            self.adj_matrix[to_index][from_index] = from_larger;
        } else  {
            self.adj_matrix[to_index][from_index] = from_larger;
            self.adj_matrix[from_index][to_index] = to_larger;
        }
    }
    fn get_edge(&self, from: usize, to: usize) -> i32{
        return self.adj_matrix[from][to];
    }
    fn traverse(&mut self, from: usize, mut visted: HashSet<usize>, hit_twice: bool) -> i32 {
        visted.insert(from);
        let paths = self.adj_matrix[from];
        let mut count = 0;
        for to in 0..paths.len() {
            let is_valid_edge = self.get_edge(from,to);

            if is_valid_edge == 1 || is_valid_edge == -1 {
                if self.get_vertex("start") == to {
                    continue;
                } else if self.get_vertex("end") == to {
                    count += 1;
                } else if is_valid_edge > 0 || !visted.contains(&to){
                    count += self.traverse(to, visted.clone(), hit_twice);
                } else if visted.contains(&to) && !hit_twice {
                    count += self.traverse(to, visted.clone(), true);
                }
            }
        }
        count
    }
}

fn main() {
    let mut graph = get_input("input.txt");
    let result = graph.traverse(graph.get_vertex("start"), HashSet::new(), true);
    println!("Part 1: {:?}", result);
    let result = graph.traverse(graph.get_vertex("start"), HashSet::new(), false);
    println!("Part 2: {:?}", result);

}
#[test]
fn part1() {
    let mut graph = get_input("input.txt");
    let result = graph.traverse(graph.get_vertex("start"), HashSet::new(), true);
    println!("Part 1: {:?}", result);
    assert_eq!(result, 5756);
}
#[test]
fn part2() {
    let mut graph = get_input("input.txt");
    let result = graph.traverse(graph.get_vertex("start"), HashSet::new(), false);
    println!("Part 2: {:?}", result);
    assert_eq!(result, 144603);
}
#[test]
fn add_edge() {
    let mut graph = Graph::new();
    graph.add_vertex("start");
    graph.add_vertex("A");
    graph.add_edge("start", "A");
    assert_eq!(graph.adj_matrix[0][1], 1);
    assert_eq!(graph.adj_matrix[1][0], -1);
}
#[test]
fn add_vertex() {
    let mut graph = Graph::new();
    graph.add_vertex("start");
    graph.add_vertex("end");

    assert_eq!(graph.get_vertex("start"), 0);
    assert_eq!(graph.get_vertex("end"), 1);
}
#[test]
fn create_graph() {
    let graph = Graph::new();
    assert_eq!(graph.vindex, 0);
    assert_eq!(graph.adj_matrix[0][0], 0);
    assert_eq!(graph.adj_matrix.len(), MAXVTEX);
}
fn get_input(filename: &str) -> Graph {
    let mut graph =Graph::new();
    if let Ok(lines) = read_lines(filename) {
        for someline in lines {
            if let Ok(line) = someline {
                let split_line = line.split("-").collect::<Vec<_>>();
                let from = split_line[0];
                let to = split_line[1];
                graph.add_vertex(from);
                graph.add_vertex(to);
                graph.add_edge(from, to);
            }
        }
    }
    graph
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}