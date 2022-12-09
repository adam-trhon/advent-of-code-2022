#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_input(text: String) -> Vec<Direction> {
    let mut result: Vec<Direction> = Vec::new();

    for line in text.split("\n") {
        if line.is_empty() {
            continue;
        }
        let str_dir: &str;
        let str_dist: &str;
        match line.split(" ").collect::<Vec<&str>>()[..] {
            [dir, dist] => {
                str_dir = dir;
                str_dist = dist;
            }
            _ => {
                println!("unparsed line: {}", line);
                continue;
            }
        }

        let dir: Direction;
        match str_dir.chars().next().unwrap() {
            'U' => dir = Direction::Up,
            'D' => dir = Direction::Down,
            'L' => dir = Direction::Left,
            'R' => dir = Direction::Right,
            _ => {
                println!("invalid direction in line {}", line);
                continue;
            }
        }

        for _ in 0..str_dist.parse::<u32>().unwrap() {
            result.push(dir.clone());
        }

    }

    result
}

fn dist(head: &(i32, i32), tail: &(i32, i32)) -> i32 {
    std::cmp::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs())
}

fn walk_the_input(input: &Vec<Direction>)-> usize {
    let mut head = (0 as i32, 0 as i32);
    let mut tail = (0 as i32, 0 as i32);

    let mut walked_tiles = std::collections::HashSet::<(i32, i32)>::new();
    walked_tiles.insert(tail);

    for step_dir in input {
        match step_dir {
            Direction::Up => head = (head.0, head.1+1),
            Direction::Down => head = (head.0, head.1-1),
            Direction::Left => head = (head.0-1, head.1),
            Direction::Right => head = (head.0+1, head.1),
        }

        if dist(&head, &tail) <= 1 {
            continue;
        }

        if head.1 > tail.1 {
            tail = (tail.0, tail.1 + 1);
        }
        if head.1 < tail.1 {
            tail = (tail.0, tail.1 - 1);
        }
        if head.0 > tail.0 {
            tail = (tail.0 + 1, tail.1);
        }
        if head.0 < tail.0 {
            tail = (tail.0 - 1, tail.1);
        }

        walked_tiles.insert(tail);
    }

    walked_tiles.len()
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let input = parse_input(text);
    let walked_tiles_count = walk_the_input(&input);
    println!("walked tiles: {}", walked_tiles_count);
}
