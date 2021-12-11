use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
#[derive(Debug)]
struct OctoMap {
    map: Vec<Octopus>,
    rows: i32,
    cols: i32,
}
impl OctoMap {
    fn reset_octopus(&mut self, row: i32, col: i32) {
        if let Some(oct) = self.map.get_mut((self.rows * row + col) as usize) {
            oct.energy = -1;
        }
    }
    fn update_octopus(&mut self, row: i32, col: i32) {
        if row >= 0 && col >= 0 && row < self.rows && col < self.cols {
            if let Some(oct) = self.map.get_mut((self.rows * row + col) as usize) {
                if oct.energy >= 0 && oct.energy < 10 {
                    oct.energy += 1;
                }
            }
        }
    }
    fn update_neighbors(&mut self, row: i32, col: i32) {
        for i in (row-1)..(row+2) {
            for j in (col-1)..(col+2) {
                self.update_octopus(i,j)
            }
        }
    }
    fn simulate_step(&mut self) -> usize {
        let mut num_flashes = 0;
        self.map.iter_mut().for_each(|o| o.energy += 1);
        loop {
            let flashes: Vec<Octopus> = self.map.clone().into_iter().filter(|c| c.energy >= 10).collect();
            num_flashes += flashes.len();
            if flashes.len() == 0 {
                break;
            }
            for flash in flashes {
                self.reset_octopus(flash.row, flash.col);
                self.update_neighbors(flash.row, flash.col);
            }
        }
        self.map.iter_mut().for_each(|o| if o.energy == -1 { o.energy = 0; });
        return num_flashes;
    }
    fn simulate_n(&mut self, num_steps: i32) {
        let mut num_flashes = 0;
        for _ in 0..num_steps {
            num_flashes += self.simulate_step();
        }
        println!("Number of Flashes: {:?}", num_flashes);
    }
    fn simulate_until_in_sync(&mut self) {
        let mut step = 1;
        while self.map.len() != self.simulate_step() {
            step += 1;
        }
        println!("First In Sync: {:?}", step);
    }

}
#[derive(Debug, Clone)]
struct Octopus {
    energy: i8,
    row: i32,
    col: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage \"cargo run <filename> <numsteps?>\"");
    }
    let filename = &args[1];
    let mut octomap = get_input(filename.as_str());
    match args.len() {
        2 => { 
            octomap.simulate_until_in_sync()
        },
        3 => {
            match args[2].parse() {
                Ok(n) => octomap.simulate_n(n),
                _ => println!("Second arg needs to be valid number!"),
            } 
        },
        _ => println!("Usage \"cargo run <filename> <numsteps?>\"")
    }
}

fn get_input(filename: &str) -> OctoMap {
    let mut map = Vec::<Octopus>::new();
    let mut r = 0;
    let mut c = 0;
    if let Ok(lines) = read_lines(filename) {
        for someline in lines {
            c = 0;
            if let Ok(line) = someline {
                for digit in line.chars() {
                    map.push(Octopus {
                        energy: digit.to_digit(10).unwrap() as i8,
                        row: r,
                        col: c
                    });
                    c += 1;
                }
            }
            r += 1;
        }
    }
    let octomap = OctoMap {
        map: map,
        rows: r,
        cols: c,
    };
    octomap
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}