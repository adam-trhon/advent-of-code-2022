use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input.txt").expect("failed to load input file");
    let input_chars: Vec::<char> = input.chars().collect();
    for i in 0..input_chars.len()-14 {
        let mut mark_detected = true;
        for j in 0..14 {
            for k in j+1..14 {
                if input_chars[i+j] == input_chars[i+k] {
                    mark_detected = false;
                }
            }
        }
        if mark_detected {
            println!("mark found after {} chars", i+14);
            break;
        }
    }
}
