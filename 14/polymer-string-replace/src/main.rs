use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
fn main() {
    part1();
    part2();
}
fn part1() {
    let (cts, pairs, mapping) = get_input("./input.txt");
    let (min_n, max_n) = max_min_chars(replace_string_n(cts, pairs, mapping, 10));
    println!("part 1: {}", max_n-min_n);
}
fn part2() {
    let (cts, pairs, mapping) = get_input("./input.txt");
    let (min_n, max_n) = max_min_chars(replace_string_n(cts, pairs, mapping, 40));
    println!("part 2: {}", max_n-min_n);
}
fn max_min_chars(cts: HashMap<String, u128>) -> (u128, u128) {
    let max_n = cts.iter().fold(0, |acc, n| cmp::max(acc, *n.1));
    let min_n = cts.iter().fold(u128::MAX, |acc, n| cmp::min(acc, *n.1));
    (min_n, max_n)
}

fn replace_string_n(mut cts: HashMap<String, u128>, mut pairs: HashMap<String, u128>, mapping: HashMap<String, String>, n:usize) -> HashMap<String, u128> {
    for i in 0..n {
        let mut new_pairs = HashMap::<String, u128>::new();
        //for all pairs that have happened, we know how many times they have happened
        for (pair, count) in pairs.into_iter() {
            if let Some(output) = mapping.get(&pair) {
                //figure out the new output char for a pair, and lets count based on 
                // count...this is a reduce. If NN hapens i times, we know c happens i times
                *cts.entry(output.to_string()).or_insert(0) += count;

                //create new pairings for next iteration. 
                //Use output and create the next left & right of output 
                let mut new_prev = pair.get(0..1).unwrap().to_string();
                let mut new_next = pair.get(1..2).unwrap();
                new_prev.push_str(output);
                let mut oclone = output.clone(); //WHY RUST WHY
                oclone.push_str(new_next);

                //This is for next iteration, count how many times a new pair shows up
                //we can then use that for calucating reduction into chars
                *new_pairs.entry(new_prev).or_insert(0) += count;
                *new_pairs.entry(oclone.to_string()).or_insert(0) += count;

            }
        }
        pairs = new_pairs;
    }
    cts
}

fn get_input(filename: &str) -> (HashMap<String, u128>, HashMap<String, u128>, HashMap<String, String>) {
    let mut input: String = "".to_string();
    let mut initial_counts: HashMap<String, u128> = HashMap::<String, u128>::new();
    let mut initial_pairs: HashMap<String, u128> = HashMap::<String, u128>::new();
    let mut mapping =HashMap::<String, String>::new();
    if let Ok(mut lines) = read_lines(filename) {
        if let Some(firstline) = lines.next() {
            input = firstline.ok().unwrap().to_string();
        }

        let mut i:usize = 0;
        while i < input.len() {
            let character = input.get(i..(i+1)).unwrap();
            //seed initial character counts
            let count = initial_counts.entry(character.to_string()).or_insert(0);
            *count +=1;
            //create initial pairings for enumberations, pair_ct is the number of times they happen
            if let Some(pair) = input.get(i..(i+2)) {
                let pair_ct = initial_pairs.entry(pair.to_string()).or_insert(0);
                *pair_ct += 1;
            }
            i += 1;
        }
        lines.next();
        for someline in lines {
            if let Ok(line) = someline {
                if let Some((key, val)) = line.split_once(" -> ") {
                    mapping.insert(key.to_string(), val.to_string());
                }
            }
        }
    }
    (initial_counts, initial_pairs, mapping)
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}