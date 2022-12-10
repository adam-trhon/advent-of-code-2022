
fn exec(text: &String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::from(vec![1;1]);

    for line in text.split("\n") {
        if line.is_empty() {
            continue;
        } else if line.starts_with("noop") {
            result.push(*result.last().unwrap());
        } else if line.starts_with("addx") {
            let current_value: i32 = *result.last().unwrap();
            let value_change: i32 = String::from(line.split_at(5).1).parse::<i32>().unwrap();
            result.push(current_value);
            result.push(current_value + value_change);
        } else {
            println!("invalid line: {}", line);
        }
    }
        

    result
}

fn draw(data: &Vec<i32>) {
    for row in data.chunks(40) {
        let mut cycle: i32 = 0;
        for pos in row {
            if (pos-cycle).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
        }
        println!("");
    }
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let value_history = exec(&input);
    draw(&value_history);
}
