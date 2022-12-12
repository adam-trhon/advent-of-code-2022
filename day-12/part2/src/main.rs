
type MapRow = Vec<u8>;
type Map = Vec<MapRow>;
type Pos = (usize, usize);

fn parse_input(input: &String) -> (Map, Pos, Pos) {
    let mut result = Map::new();
    let mut current: Option<Pos> = None;
    let mut target: Option<Pos> = None;

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut row = MapRow::new();

        for height in line.chars() {
            let height_real = match height {
                'S' => {
                    current = Some((result.len(), row.len()));
                    'a'
                }
                'E' => {
                    target = Some((result.len(), row.len()));
                    'z'
                }
                c => c
            };

            row.push((height_real as u8) - ('a' as u8));
        }

        result.push(row);
    }

    (result, current.expect("current pos not detected"), target.expect("target pos not detected"))
}

fn gen_reachable_from(map: &Map, pos: &Pos) -> std::collections::HashSet<Pos> {
    let mut surroundings = std::collections::HashSet::<Pos>::new();
    if pos.1 > 0 {
        surroundings.insert((pos.0, pos.1-1));
    }
    if pos.1 < map[pos.0].len()-1 {
        surroundings.insert((pos.0, pos.1+1));
    }
    if pos.0 > 0 {
        surroundings.insert((pos.0-1, pos.1));
    }
    if pos.0 < map.len()-1 {
        surroundings.insert((pos.0+1, pos.1));
    }

    let mut reachable_from = std::collections::HashSet::<Pos>::new();
    for neighbour in surroundings {
        if (map[pos.0][pos.1] as i32 - map[neighbour.0][neighbour.1] as i32) <= 1 {
            reachable_from.insert(neighbour);
            /*
            println!("{:?} [{}] reachable from {:?} [{}]",
                neighbour, map[neighbour.0][neighbour.1],
                pos, map[pos.0][pos.1]);
            */
        } else {
            /*
            println!("{:?} [{}] not reachable from {:?} [{}]",
                neighbour, map[neighbour.0][neighbour.1],
                pos, map[pos.0][pos.1]);
            */
        }
    }

    reachable_from
}

fn find_path(map: &Map, _start: &Pos, goal: &Pos) -> u32 {
    let mut path_length: u32 = 0;
    let mut explored = std::collections::HashSet::<Pos>::new();
    let mut just_explored = std::collections::HashSet::<Pos>::new();

    just_explored.insert(*goal);

    'map_search: loop {
        path_length += 1;
        let mut next_explored = std::collections::HashSet::<Pos>::new();
        for pos in just_explored {
            let reachable = gen_reachable_from(map, &pos);
            next_explored = next_explored.union(&reachable).map(|v| *v).collect();
        }
        for pos in next_explored.iter() {
            if map[pos.0][pos.1] == 0 {
                break 'map_search;
            }
        }

        just_explored = next_explored.difference(&explored).map(|v| *v).collect();
        explored = explored.union(&next_explored).map(|v| *v).collect();
    }

    path_length
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let (map, current, target) = parse_input(&input);
    let path_length = find_path(&map, &current, &target);
    println!("path length: {}", path_length);
}
