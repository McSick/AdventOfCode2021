use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut first = 0;
    let mut second = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let nums: Vec<u32> = lines.into_iter().map(|l| {
            l.ok().and_then(|s| s.parse().ok()).unwrap_or(0)
        }).collect();
        first = find_increaseing_first(&nums);
        second = find_increaseing_second(&nums);
    }
    println!("First: {} Second: {}", first, second);
}

fn find_increaseing_first(nums: &Vec<u32>) -> u32 {
    let mut increment = 0;
    for i in 0..nums.len() - 3 {
        if nums[i] < nums[i+1] { 
            increment += 1;
        }
    }
    return increment;
}
fn find_increaseing_second(nums: &Vec<u32>) -> u32 {
    let mut increment = 0;
    for i in 0..nums.len() - 3 {
        let first = nums[i] + nums[i+1] + nums[i+2];
        let second = nums[i+1] + nums[i+2] + nums[i+3];
        if first < second {
            increment += 1;
        }
    }
    return increment;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}