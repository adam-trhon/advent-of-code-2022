
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

fn map_visibility(map: &Map) -> Map {
    let mut visibility = Map::from(vec![Row::from(vec![0; map[0].len()]); map.len()]);

    // left to right
    for i in 0..map.len() {
        let mut current_max_height: i32 = -1;
        for j in 0..map[0].len() {
            if map[i][j] as i32 > current_max_height {
                visibility[i][j] = 1;
                current_max_height = map[i][j] as i32;
            }
        }
    }

    // right to left
    for i in 0..map.len() {
        let mut current_max_height: i32 = -1;
        for j in (0..map[0].len()).rev() {
            if map[i][j] as i32 > current_max_height {
                visibility[i][j] = 1;
                current_max_height = map[i][j] as i32;
            }
        }
    }

    // top to bottom
    for j in 0..map[0].len() {
        let mut current_max_height: i32 = -1;
        for i in 0..map.len() {
            if map[i][j] as i32 > current_max_height {
                visibility[i][j] = 1;
                current_max_height = map[i][j] as i32;
            }
        }
    }

    // bottom to top
    for j in 0..map[0].len() {
        let mut current_max_height: i32 = -1;
        for i in (0..map.len()).rev() {
            if map[i][j] as i32 > current_max_height {
                visibility[i][j] = 1;
                current_max_height = map[i][j] as i32;
            }
        }
    }

    visibility
}

fn count_visible(visibility: &Map) -> u32 {
    visibility.into_iter().map(|row| row.into_iter().map(|tile| *tile as u32).sum::<u32>()).sum()
}

fn main() {
    let input_text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let map = parse_input(&input_text);
    println!("loaded map {} x {}", map.len(), map[0].len());
    let visibility = map_visibility(&map);
    let visible_count = count_visible(&visibility);
    println!("visible trees: {}", visible_count);
}
