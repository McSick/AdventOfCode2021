use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
#[derive(Debug)]
struct Problem {
    patterns:Vec<String>,
    outputs:Vec<String>
}
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let problems = get_input("input.txt");

    //Part 1
    let problem_num = &args[1];
    if problem_num == "1" {
        let unique = count_unique_outputs(problems);
        println!("Answer: {}", unique);
    } else { 
        let summed_digits = decode_digits(problems);
        println!("Answer: {}", summed_digits);
    }
    
}


fn decode_digits(problems:Vec<Problem>) -> i32 {
    let mut sum = 0;
    for problem in problems {
        sum += solve_problem(problem); 
    }
    sum
}
fn solve_problem(p: Problem) -> i32 {
    let mut char2digit = HashMap::new();
    let mut lencounts =  HashMap::new();
    let mut digit2char =  HashMap::new();
    // Loop through 10 patterns
    for display in p.patterns {
        let len = display.len();
        //map the lengths, can be used to decrypt for len 2 & 4 #s(1 & 4)
        lencounts.insert(len, display.clone());

        //count the number of times a segment shows up, can be used to decrypt
        for segmentchar in display.chars() {
            *char2digit.entry(segmentchar).or_insert(0) += 1;
        }
    }

    //ac and dg share the number of times a segment shows up ac -> 8, dg ->7
    let mut ac = "".to_string();
    let mut dg = "".to_string();
    for (key, value) in char2digit.into_iter() {
        //We want to save these to reduce later
        if value == 8 {
            ac += &key.to_string();
        }
        if value == 7 {
            dg += &key.to_string();
        }
        digit2char.insert(value, key);
    }

    let mut a ='x';
    let mut c ='x';
    let mut d ='x';
    let mut g ='x';
    // #1 has 2 segments. 1 of which is c but not a so we can loop through ac 
    // a wont show up in the displaystr but c will
    let displaystr = lencounts.entry(2).or_insert("".to_string());
    for potential_char in ac.chars() {
        
        if displaystr.contains(potential_char) {
            c = potential_char;
        } else { 
            a = potential_char;
        }
    }

    // #4 has 4 segments. 1 of which is d but not gso we can loop through dg 
    // d will show up but g wont
    let displaystr = lencounts.entry(4).or_insert("".to_string());
    for potential_char in dg.chars() {
        if displaystr.contains(potential_char) {
            d = potential_char;
        } else { 
            g = potential_char;
        }
    }

    // these all map to unique #of times a segment will show up across all 10 digits
    let b = *digit2char.entry(6).or_insert('x');
    let e = *digit2char.entry(4).or_insert('x');
    let f = *digit2char.entry(9).or_insert('x');
    
    // so we can now map characters to what they really are. 
    let charmap = hashmap![a => 'a', b => 'b', c => 'c', d => 'd', e => 'e', f => 'f', g => 'g'];

    let mut result = 0;
    for word in p.outputs {
        // change into the real mapping
        let newword = substitute(word, &charmap);
        // decrypt based on real values
        let num = get_num_from_segments(newword);
        result = result * 10 + num;
    } 
    result
}

fn get_num_from_segments(newword:String) -> i32 {
    let segments_to_num = hashmap![
    "abcefg".to_string() => 0, 
    "cf".to_string() => 1, 
    "acdeg".to_string() => 2, 
    "acdfg".to_string() => 3, 
    "bcdf".to_string() => 4, 
    "abdfg".to_string() => 5, 
    "abdefg".to_string() => 6, 
    "acf".to_string() => 7,
    "abcdefg".to_string() => 8,
    "abcdfg".to_string() => 9];
    return *segments_to_num.get(&newword).unwrap_or(&0);

}
/**
 * This will take the mixed up value and change it to the real value.
 * we sort it in increasing order at the end do it can properly mapped 
 * to segement to num
 */
fn substitute(word:String, charmap:&HashMap<char,char>) -> String {
    let mut newword:String = "".to_string();
    for single_digit in word.chars() {
            let newchar = charmap.get(&single_digit).unwrap_or(&'x');
            newword += &newchar.to_string();
    }
    let mut chars: Vec<char> = newword.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    let s = String::from_iter(chars);
    s
}

fn count_unique_outputs(problems:Vec<Problem>) -> i32 {
    let uniqueset =  HashSet::from([2,3,4,7]);
    let mut count = 0;
    for problem in problems {
        for num in problem.outputs {
            if uniqueset.contains(&num.len()) {
                count += 1;
            }
        }
    }
    count
}
fn get_input(filename: &str) -> Vec<Problem> {
    let mut problems = Vec::<Problem>::new();
    if let Ok(lines) = read_lines(filename) {
        for someline in lines {
            if let Ok(line) = someline {
                let split_line = line.split("|").collect::<Vec<_>>();
                let patterns = split_line[0].split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
                let outputs = split_line[1].split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
                problems.push(Problem{
                    patterns: patterns,
                    outputs: outputs
                });
            }
        }
    }
    problems
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[test]
fn test_solve_problem() {
    let p = Problem{
        patterns: vec!["acedgfb".to_string(), "cdfbe".to_string(), "gcdfa".to_string(), "fbcad".to_string(), "dab".to_string(), "cefabd".to_string(), "cdfgeb".to_string(), "eafb".to_string(), "cagedb".to_string(), "ab".to_string()],
        outputs: vec!["cdfeb".to_string(), "fcadb".to_string(), "cdfeb".to_string(), "cdbaf".to_string()]
    };
    let answer = solve_problem(p);
    assert_eq!(answer, 5353, "Problem was not 5353, returned `{}`", answer);

}
#[test]
fn test_decode_digits() {
    let problems = get_input("simple.txt");
    let answer = decode_digits(problems);
    assert_eq!(answer, 61229, "Decode digits was not 61229, answer was `{}`", answer);
}
#[test]
fn test_count_unique() {
    let problems = get_input("simple.txt");
    let answer = count_unique_outputs(problems);
    assert_eq!(answer, 26, "Count unique was not 26, answer was `{}`", answer);
}
