use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;

mod graph;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let edges = get_input(filename.as_str());

    let visted: HashSet<String> = HashSet::new();
    let result = traverse("start".to_string(), edges.clone(), visted.clone(), true);
    println!("{:?}", result);
    let result = traverse("start".to_string(), edges, visted, false);
    println!("{:?}", result);
}
fn is_large(name:String) -> bool {
    let mut chars = name.chars();
    let mut is_large = false;
    if let Some(firstchar) = chars.next() {
        is_large = firstchar.is_ascii_uppercase();
    }
    is_large
}
fn traverse(curr: String, treemap: HashMap<String, Vec<String>>, mut visted: HashSet<String>, has_traverse_twice:bool) -> i32 {
    visted.insert(curr.clone());
    let mut count = 0;
    if let Some(node) = treemap.get(&curr) {
        let mut paths = node.clone();
        while paths.len() > 0 {
            if let Some(next) = paths.pop() {
                if next.eq("start") {
                    continue;
                } else if next.eq("end") {
                    count += 1;
                } else if is_large(next.clone()) {
                    count += traverse(next.clone(), treemap.clone(), visted.clone(), has_traverse_twice);
                } else if !visted.contains(&next) {
                    count += traverse(next.clone(), treemap.clone(), visted.clone(), has_traverse_twice);
                } else if visted.contains(&next) && !has_traverse_twice {
                    count += traverse(next.clone(), treemap.clone(), visted.clone(), true);
                }
      
            }
        }
    }
    count
}

fn get_input(filename: &str) -> HashMap<String, Vec<String>> {
    let mut edgemap = HashMap::<String, Vec<String>>::new();
    if let Ok(lines) = read_lines(filename) {
        for someline in lines {
            if let Ok(line) = someline {
                let split_line = line.split("-").collect::<Vec<_>>();
                let left = split_line[0].to_string();
                let right = split_line[1].to_string();

                if left.eq("start") {
                    let entry = edgemap.entry(left.clone()).or_insert(Vec::<String>::new());
                    entry.push(right);
                }
                else if right.eq("start") {
                    let entry = edgemap.entry(right.clone()).or_insert(Vec::<String>::new());
                    entry.push(left);
                }
                else if right.eq("end") {
                    let entry = edgemap.entry(left.clone()).or_insert(Vec::<String>::new());
                    entry.push(right);
                }
                else if left.eq("end") {
                    let entry = edgemap.entry(right.clone()).or_insert(Vec::<String>::new());
                    entry.push(left);
                } else {
                    let entry = edgemap.entry(right.clone()).or_insert(Vec::<String>::new());
                    entry.push(left.clone());
                    let entry = edgemap.entry(left.clone()).or_insert(Vec::<String>::new());
                    entry.push(right.clone());
                }
            }
        }
    }
    edgemap
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}