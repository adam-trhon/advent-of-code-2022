
type Row = Vec<u8>;
type Map = Vec<Row>;

fn draw_line(map: &mut Map, a: &(u32, u32), b: &(u32, u32)) {
    if a.0 == b.0 {
        let from = std::cmp::min(a.1, b.1);
        let to = std::cmp::max(a.1, b.1);
        for i in from..=to {
            map[i as usize][a.0 as usize] = 1;
        }
    } else if a.1 == b.1 {
        let from = std::cmp::min(a.0, b.0);
        let to = std::cmp::max(a.0, b.0);
        for i in from..=to {
            map[a.1 as usize][i as usize] = 1;
        }
    } else {
        panic!("diagonal line");
    }
}

fn parse_input(text: &String) -> Map {
    let mut result = Map::new();
    for _ in 0..1000 {
        result.push(Row::from([0; 1000]));
    }

    let mut coord_row_max = 0;

    for line in text.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut coords = Vec::<(u32, u32)>::new();
        for pair in line.split(" -> ") {
            let mut numbers_str = pair.split(",");
            coords.push((
                numbers_str.next().unwrap().parse().unwrap(),
                numbers_str.next().unwrap().parse().unwrap(),
            ));
        }

        let mut coord_it = coords.iter();
        let mut coord_prev = coord_it.next().unwrap();
        coord_row_max = std::cmp::max(coord_row_max, coord_prev.1);
        loop {
            match coord_it.next() {
                None => break,
                Some(coord) => {
                    draw_line(& mut result, &coord_prev, &coord);
                    coord_prev = coord;
                    coord_row_max = std::cmp::max(coord_row_max, coord_prev.1);
                }
            }
        }
    }

    let floor: usize = (coord_row_max+2) as usize;
    for i in 0..result[floor].len() {
        result[floor][i] = 1;
    }

    result
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    let row_begin: usize = 0;
    let row_end: usize = 14;
    let col_begin: usize = 483;
    let col_end: usize = 514;

    for r in row_begin..=row_end {
        for c in col_begin..=col_end {
            match map[r][c] {
                0 => print!("."),
                1 => print!("#"),
                2 => print!("o"),
                _ => panic!("unknown value in map"),
            }
        }
        println!("");
    }
    println!("");
}

fn simulate_grain(map: &mut Map) -> bool {
    let mut grain_pos = (500 as usize, 0 as usize);

    //print_map(map);

    if map[grain_pos.1][grain_pos.0] != 0 {
        return true;
    }

    loop {
        if grain_pos.1 + 1 >= map.len() {
            panic!("this should never happen now!");
        } else if map[grain_pos.1+1][grain_pos.0] == 0 {
            grain_pos.1 += 1;
            continue;
        } else if map[grain_pos.1+1][grain_pos.0-1] == 0 {
            grain_pos.1 += 1;
            grain_pos.0 -= 1;
            continue;
        } else if map[grain_pos.1+1][grain_pos.0+1] == 0 {
            grain_pos.1 += 1;
            grain_pos.0 += 1;
            continue;
        } else {
            map[grain_pos.1][grain_pos.0] = 2;
            return false;
        }
    }
}

fn fill_with_grains(map: &mut Map) -> u32 {
    let mut grains: u32 = 0;

    while !simulate_grain(map) {
        grains += 1;
    }

    grains
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("cannot read input file");
    let mut map = parse_input(&text);
    println!("grain capacity: {}", fill_with_grains(&mut map));
}
