use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
fn main() {
    println!("Hello, world!");
    let mut graph = get_input("input.txt");
    let cost = dijkstra(graph.clone(), (0,0));
    println!("Part1: {}", cost[cost.len() - 1][cost.len() - 1]);
    graph = make_larger(graph);
    let cost2 = dijkstra(graph, (0,0));
    println!("Part2: {}", cost2[cost2.len() - 1][cost2.len() - 1]);
    //pretty_print(graph);   
}
fn _pretty_print(graph: Vec<Vec<u32>>) {
    for y in 0..graph.len() {
        for x in 0..graph.len() {
            print!("{} ", graph[y][x]);
        }
        println!("");
    }
}
fn get_neighors(point: (usize, usize), graphsize: usize) -> Vec<(usize, usize)> {
    let mut points = vec![];
    let ipoint = (point.0 as isize, point.1 as isize);
    let deltas: Vec<(isize,isize)> = vec![(-1, 0), (1, 0), (0, -1), (0,1)];
    for delta in deltas {
        let newpt = (ipoint.0 + delta.0, ipoint.1+delta.1);
        if newpt.0 >= 0 && newpt.0 < (graphsize as isize) && newpt.1 >= 0 && newpt.1 < (graphsize as isize) {
            points.push((newpt.0 as usize, newpt.1 as usize));
        }
    }
    points

}
fn make_larger(mut graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let graph_len = graph.len();
    let new_len = graph_len * 5;
    for r in 0..graph_len {
        for c in graph_len..new_len {
            let new_value = if graph[r][c-graph_len] == 9 { 1 } else {graph[r][c-graph_len] + 1 };
            graph[r].push(new_value);
        }
    }

    for r in graph_len..new_len {
        graph.push(vec![]);
        for c in 0..new_len {
            let new_value = if graph[r-graph_len][c] == 9 { 1 } else { graph[r-graph_len][c] + 1 };
            graph[r].push(new_value);
         
        }
    }
    graph
}

fn dijkstra(graph: Vec<Vec<u32>>, start: (usize, usize) ) -> Vec<Vec<u32>> {
    let graphsize = graph.len();
    let mut cost = vec![vec![u32::MAX; graph.len()]; graph.len()];
    cost[start.0][start.1] = 0;
    let mut pq = PriorityQueue::<(usize,usize), Reverse<u32>>::new();
    let mut visted = HashSet::<(usize, usize)>::new();
    pq.push(start, Reverse(0 as u32));
    while !pq.is_empty() {
        if let Some((current, _dist)) = pq.pop() {
            visted.insert(current);
            let mut neighbors = get_neighors(current, graphsize);
            neighbors.sort_unstable_by(|a,b| graph[a.0][a.1].partial_cmp(&graph[b.0][b.1]).unwrap());
            for neighbor in neighbors {
                let distance = graph[neighbor.0][neighbor.1];
                if !visted.contains(&neighbor) {
                    let old_cost = cost[neighbor.0][neighbor.1];
                    let new_cost = cost[current.0][current.1] + distance;
                    if new_cost < old_cost {
                        pq.push(neighbor, Reverse(new_cost));
                        cost[neighbor.0][neighbor.1] = new_cost;
                    }
                }
            }
        }
    }
    cost
}

fn get_input(filename: &str) -> Vec<Vec<u32>> {
    let mut graph = vec![];
    if let Ok(lines) = read_lines(filename) {        
        graph = lines.into_iter().map(|l| l.unwrap().chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    }
    graph
}
#[test]
fn test_input() {
    let graph = get_input("test.txt");
    assert_eq!(graph[0][2], 6);
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}