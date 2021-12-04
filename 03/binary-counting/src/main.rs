use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let all_numbers: Vec<String> = lines.into_iter().map(|l| l.ok().unwrap()).collect();
        let trackingarr = get_most_common_digits(all_numbers.clone());
        let mut gamma = 0;
        let mut epsilon = 0;
        let mut nthpower = 12;

        for i in 0..trackingarr.len() {
            nthpower -= 1;
            if trackingarr[i] > 0 {
                gamma += i32::pow(2, nthpower);
            } else {
                epsilon += i32::pow(2, nthpower);
            }
        }
        println!("G: {} E: {} Mul: {}", gamma, epsilon, gamma * epsilon);
        let oxy = oxygen_generator_rating(all_numbers.clone(), trackingarr, 0);
        let scrubby = scrubber_rating(all_numbers.clone(), trackingarr, 0);
        println!("O: {} C: {} Mul: {}", oxy, scrubby, oxy * scrubby);
    }
}
fn get_most_common_digits(potential_numbers: Vec<String>) -> [i32; 12] {
    let mut trackingarr: [i32; 12] = [0; 12];
    for num in potential_numbers {
        let chararray = num.as_bytes();
        for i in 0..chararray.len() {
            match chararray[i] as char {
                '0' => trackingarr[i] -= 1,
                '1' => trackingarr[i] += 1,
                _ => println!("Oh no"),
            }
        }
    }
    return trackingarr;
}
fn convert2num(binstring: String) -> i32 {
    let mut nthpower = 12;
    let mut result = 0;
    for c in binstring.chars() {
        nthpower -= 1;
        if c == '1' {
            result += i32::pow(2, nthpower);
        }
    }
    return result;
}
//takes in the current slice, most common,  & current index
fn oxygen_generator_rating(
    mut potential_numbers: Vec<String>,
    most_common_bit_array: [i32; 12],
    current_bit_index: usize,
) -> i32 {
    //Find most common bit in curr pos
    let most_common_bit = if most_common_bit_array[current_bit_index] >= 0 {
        '1'
    } else {
        '0'
    };
    potential_numbers.retain(|num| num.as_bytes()[current_bit_index] as char == most_common_bit);
    if potential_numbers.len() == 1 {
        return convert2num(potential_numbers[0].clone());
    } else if (current_bit_index + 1) >= 12 {
        return -1;
    } else {
        return oxygen_generator_rating(
            potential_numbers.clone(),
            get_most_common_digits(potential_numbers.clone()),
            current_bit_index + 1,
        );
    }
}

fn scrubber_rating(
    mut potential_numbers: Vec<String>,
    most_common_bit_array: [i32; 12],
    current_bit_index: usize,
) -> i32 {
    let least_common_bit = if most_common_bit_array[current_bit_index] < 0 {
        '1'
    } else {
        '0'
    };
    potential_numbers.retain(|num| num.as_bytes()[current_bit_index] as char == least_common_bit);
    if potential_numbers.len() == 1 {
        return convert2num(potential_numbers[0].clone());
    } else if (current_bit_index + 1) >= 12 {
        return -1;
    } else {
        scrubber_rating(
            potential_numbers.clone(),
            get_most_common_digits(potential_numbers.clone()),
            current_bit_index + 1,
        )
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
