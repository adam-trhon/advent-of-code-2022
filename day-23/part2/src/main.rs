
type Coords = (i32, i32);

type ElfMap = std::collections::HashSet<Coords>;

fn parse_input(text: &str) -> ElfMap {
    let mut result = ElfMap::new();

    for (row, line) in text.split("\n").enumerate() {
        for (col, mark) in line.chars().enumerate() {
            if mark == '#' {
                result.insert((-(row as i32), col as i32));
            }
        }
    }

    result
}

fn make_step(map: &mut ElfMap, first_proc: u8) -> bool {
    let mut moved = false;

    type Moves = std::collections::HashMap<Coords, Coords>;
    let mut moves = Moves::new();

    type Propositions = std::collections::HashMap<Coords, usize>;
    let mut propositions = Propositions::new();

    for coords in map.iter() {
        let next_coords = propose_step(coords, &map, first_proc);

        moves.insert(*coords, next_coords);
        propositions.entry(next_coords).and_modify(|count| *count += 1).or_insert(1);
    }

    map.clear();
    
    for (from, to) in moves.into_iter() {
        if *propositions.get(&to).unwrap() == 1 && to != from{
            map.insert(to);
            moved = true;
        } else {
            map.insert(from);
        }
    }

    moved
}

fn propose_step(coords: &Coords, map: &ElfMap, first_proc: u8)  -> Coords {
    let n = (coords.0+1, coords.1);
    let s = (coords.0-1, coords.1);
    let w = (coords.0, coords.1-1);
    let e = (coords.0, coords.1+1);
    let ne = (coords.0+1, coords.1+1);
    let se = (coords.0-1, coords.1+1);
    let nw = (coords.0+1, coords.1-1);
    let sw = (coords.0-1, coords.1-1);

    let mut moves = false;
    for c in [n, s, w, e, ne, se, nw, sw] {
        if map.contains(&c) {
            moves = true;
            break;
        }
    }
    if !moves {
        return *coords;
    }

    for procedure in (first_proc..first_proc+4).map(|p| p%4) {
        match procedure {
            0 => {
                if !map.contains(&ne) && !map.contains(&n) && !map.contains(&nw) {
                    return n;
                }
            }
            1 => {
                if !map.contains(&se) && !map.contains(&s) && !map.contains(&sw) {
                    return s;
                }
            }
            2 => {
                if !map.contains(&nw) && !map.contains(&w) && !map.contains(&sw) {
                    return w;
                }
            }
            3 => {
                if !map.contains(&ne) && !map.contains(&e) && !map.contains(&se) {
                    return e;
                }
            }
            _ => {
                panic!("invalid procedure");
            }
        }
    }
    *coords
}

#[allow(dead_code)]
fn draw(map: &ElfMap) {
    let mut max_n = i32::MIN;
    let mut max_s = i32::MAX;
    let mut max_e = i32::MIN;
    let mut max_w = i32::MAX;

    for (ver, hor) in map {
        if hor < &max_w {
            max_w = *hor;
        }
        if hor > &max_e {
            max_e = *hor;
        }
        if ver < &max_s {
            max_s = *ver;
        }
        if ver > &max_n {
            max_n = *ver;
        }
    }

    for v in (max_s..=max_n).rev() {
        for h in max_w..=max_e {
            if map.contains(&(v, h)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let mut map = parse_input(&text[..]);
    let mut round: i32 = 0;
    while make_step(&mut map, (round%4) as u8) {
        if round % 100 == 0 {
            println!("round {}....", round);
        }
        round += 1;
    }
    round += 1;

    println!("finished at round {}", round);
}
