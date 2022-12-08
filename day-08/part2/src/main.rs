
type Row = Vec<u8>;
type Map = Vec<Row>;

fn parse_input(text: &String) -> Map {
    let mut result = Map::new();
    for line in text.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let row: Row = line.as_bytes().to_vec();
        result.push(row);
    }
    result
}

fn count_tree_score(row: &usize, col: &usize, map: &Map) -> u32 {
    let current_height = map[*row][*col];
    let mut current_score: u32;
    let mut total_score: u32 = 1;

    if *row == 0 || *col == 0 || *row == map.len()-1 || *col == map[0].len() - 1 {
        return 0;
    }

    // to west
    current_score = 0;
    for j in (0..*col).rev() {
        current_score += 1;
        if map[*row][j] >= current_height {
            break;
        }
    }
    total_score *= current_score;

    // to east
    current_score = 0;
    for j in *col+1..map[0].len() {
        current_score += 1;
        if map[*row][j] >= current_height {
            break;
        }
    }
    total_score *= current_score;

    // to north
    current_score = 0;
    for i in (0..*row).rev() {
        current_score += 1;
        if map[i][*col] >= current_height {
            break;
        }
    }
    total_score *= current_score;

    // to south
    current_score = 0;
    for i in *row+1..map.len() {
        current_score += 1;
        if map[i][*col] >= current_height {
            break;
        }
    }
    total_score *= current_score;

    total_score
}

fn find_best_tree_score(map: &Map) -> u32 {
    let mut best_score: u32 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let current_score = count_tree_score(&i, &j, map);
            if current_score > best_score {
                best_score = current_score;
            }
        }
    }
    best_score
}

fn main() {
    let input_text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let map = parse_input(&input_text);
    println!("loaded map {} x {}", map.len(), map[0].len());
    println!("best tree score: {}", find_best_tree_score(&map));
}
