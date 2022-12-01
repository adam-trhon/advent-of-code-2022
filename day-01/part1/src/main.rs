
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Failed to read input file");
    let lines = contents.split("\n");

    let mut current_elf = 0;
    let mut best_elf = 0;

    for line in lines {
        if line.is_empty() {
            if best_elf < current_elf {
                best_elf = current_elf;
            }
            current_elf = 0;
        } else {
            let current_meal : u32 = line.parse().unwrap();
            current_elf += current_meal;
        }
    }

    println!("best elf: {}", best_elf);
}
