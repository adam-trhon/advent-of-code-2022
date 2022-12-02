use std::fs;
use std::collections::HashMap;

fn build_score_map() -> HashMap<String, u32> {
    let mut scores = HashMap::new();

    let rock : u32 = 1;
    let paper : u32 = 2;
    let scissors : u32 = 3;
    let win : u32 = 6;
    let draw : u32 = 3;
    let loss : u32 = 0;
    
    // his rock vs my
    scores.insert(String::from("A X"), draw+rock); // rock
    scores.insert(String::from("A Y"), win+paper); // paper
    scores.insert(String::from("A Z"), loss+scissors); // scissors
    // his paper vs my
    scores.insert(String::from("B X"), loss+rock);
    scores.insert(String::from("B Y"), draw+paper);
    scores.insert(String::from("B Z"), win+scissors);
    // his scissors vs my
    scores.insert(String::from("C X"), win+rock);
    scores.insert(String::from("C Y"), loss+paper);
    scores.insert(String::from("C Z"), draw+scissors);

    scores
}

fn main() {
    let content = fs::read_to_string("../input.txt").expect("Failed to read input file");
    let lines = content.split("\n");

    let scores = build_score_map();
    let mut my_score = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }
        my_score += scores.get(line).expect("failed to find in map");
    }
    println!("my total score: {}", my_score);
}
