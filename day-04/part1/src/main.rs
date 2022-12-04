use std::fs::read_to_string;
use regex::Regex;

type IdRange = std::ops::Range<u32>;

fn parse_input(input: String) -> Vec::<(IdRange, IdRange)>  {

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut result = Vec::<(IdRange, IdRange)>::new();

    for c in re.captures_iter(&input) {
        result.push((
            IdRange{start: c[1].parse().unwrap(), end: c[2].parse().unwrap()},
            IdRange{start: c[3].parse().unwrap(), end: c[4].parse().unwrap()},
        ));
    }

    result
}

fn count_overlaps(pairs: &Vec::<(IdRange, IdRange)>) -> u32 {
    let mut overlaps: u32 = 0;

    for pair in pairs {
        if (pair.0.start <= pair.1.start) && (pair.0.end >= pair.1.end) {
            overlaps += 1;
        } else if (pair.1.start <= pair.0.start) && (pair.1.end >= pair.0.end) {
            overlaps += 1;
        }
    }

    overlaps
}

fn main() {
    let input = read_to_string("../input.txt").expect("failed to read input");
    let pairs = parse_input(input);
    let overlaps = count_overlaps(&pairs);
    println!("overlaps: {}", overlaps);
}
