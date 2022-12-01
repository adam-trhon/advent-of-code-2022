
use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("Failed to read input file");
    let lines = contents.split("\n");

    let mut current_elf = 0;
    let mut best_elves: [u32; 3] = [0; 3];

    for line in lines {
        if line.is_empty() {
            if best_elves[0] < current_elf {
                best_elves[0] = current_elf;
                best_elves.sort()
            }
            current_elf = 0;
        } else {
            let current_meal : u32 = line.parse().unwrap();
            current_elf += current_meal;
        }
    }

    if current_elf > 0 {
        println!("last elf not processed");
    }

    let best_elves_total : u32 = best_elves.iter().sum();

    println!("total of best elves: {}", best_elves_total);
}
