use std::fs::read_to_string;
use regex::Regex;

type Stack = Vec<char>;
type Storage = Vec<Stack>;
struct CraneStep {
    count: u32,
    from: usize, 
    to: usize,
}

type CraneInput = Vec<CraneStep>;

fn build_storage_regex(line: &str) -> (usize, String) {
    let mut storage_regex = String::from(r"(\[.\]|   )");
    let stacks = (line.len()+1)/4;
    for _ in 0..stacks-1 {
        storage_regex = storage_regex + r" (\[.\]|   )";
    }

    (stacks, storage_regex)
}

fn parse_input(input: String) -> (Storage, CraneInput) {

    let mut lines = input.split("\n");
    let (stacks, storage_regex_str) = build_storage_regex(&lines.next().unwrap());
    let mut storage = Storage::new();
    for _ in 0..stacks {
        storage.push(Stack::new());
    }

    let storage_regex = Regex::new(&storage_regex_str).unwrap();

    for stack_line in storage_regex.captures_iter(&input) {
        for i in 0..stacks {
            let mut chars = stack_line[i+1].chars();
            if chars.next().unwrap() == '[' {
                storage[i].insert(0, chars.next().unwrap());
            }
        }
    }

    let mut crane_input = CraneInput::new();
    let crane_input_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for input_line in crane_input_regex.captures_iter(&input) {
        crane_input.push(CraneStep{
            count: input_line[1].parse().unwrap(),
            from: input_line[2].parse().unwrap(),
            to: input_line[3].parse().unwrap(),
        });
    }

    (storage, crane_input)
}

fn perform_step(storage: & mut Storage, step: &CraneStep) {
    let to_pos = storage[step.to-1].len();
    for _ in 0..step.count {
        let item = storage[step.from-1].pop().unwrap();
        storage[step.to-1].insert(to_pos, item);
    }
}

fn main() {
    let input = read_to_string("../input.txt").expect("failed to read input file");
    let (mut storage, crane_input) = parse_input(input);

    for step in crane_input {
        perform_step(&mut storage, &step);
    }

    for mut stack in storage {
        print!("{}", stack.pop().unwrap());
    }
    println!("");
}
