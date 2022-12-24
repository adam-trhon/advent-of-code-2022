
#[derive(Clone,Copy,Debug)]
enum Dir {
    North,
    West,
    South,
    East,
}
type Position = (usize, usize);
type Blizzards = multimap::MultiMap<Position, Dir>;


fn parse_input(text: &String) -> (Blizzards, (usize, usize)) {
    let mut result = Blizzards::new();
    let mut rows: usize = 0;
    let cols: usize;

    for (row,line) in text.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        rows = row+1;
        if line.chars().nth(1).unwrap() == '#' {
            continue
        }

        result.extend(line.chars().enumerate().filter_map(|(col, c)| match c {
            '#' => None,
            '.' => None,
            '^' => Some(((row, col), Dir::North)),
            '<' => Some(((row, col), Dir::West)),
            'v' => Some(((row, col), Dir::South)),
            '>' => Some(((row, col), Dir::East)),
            _ => panic!("invalid char in map"),
        }));
    }

    if rows > 0 {
        cols = text.split("\n").next().unwrap().len();
    } else {
        cols = 0;
    }

    (result, (rows, cols))
}

type MapSnapshot = Vec<Vec<char>>;

struct LazyMap {
    blizzards: Blizzards,
    snapshots: Vec<MapSnapshot>,
    rows: usize,
    cols: usize,
}

impl LazyMap {
    fn new(blizzards: Blizzards, size: (usize, usize)) -> Self {
        let mut result = Self{
            snapshots: Vec::<MapSnapshot>::new(),
            blizzards: blizzards,
            rows: size.0,
            cols: size.1,
        };
        result.take_snapshot();
        result
    }

    fn take_snapshot(&mut self) {
        let mut snapshot = MapSnapshot::new();

        for row in 0..self.rows {
            let mut row_data = vec!['.'; self.cols];
            for col in 0..self.cols {
                if row == 0 && col != 1 {
                    row_data[col] = '#';
                } else if row+1 == self.rows && col+2 != self.cols {
                    row_data[col] = '#';
                } else if col == 0 || col+1 == self.cols  {
                    row_data[col] = '#';
                } 
            }
            snapshot.push(row_data);
        }

        for pos in self.blizzards.keys() {
            let b = self.blizzards.get_vec(pos).unwrap();
            if b.len() == 1 {
                match b[0] {
                    Dir::North => { snapshot[pos.0][pos.1] = '^'},
                    Dir::West => { snapshot[pos.0][pos.1] = '<'},
                    Dir::East => { snapshot[pos.0][pos.1] = '>'},
                    Dir::South => { snapshot[pos.0][pos.1] = 'v'},
                }
            } else if b.len() > 4 {
                panic!("too many blizzards");
            } else {
                snapshot[pos.0][pos.1] = ['0', '1', '2', '3', '4'][b.len()];
            }
        }

        self.snapshots.push(snapshot);

        //self.draw_snapshot(self.snapshots.len()-1);
    }

    fn advance_blizzards(&mut self) {
        let mut next_blizzards = Blizzards::new();

        // Fuck multimaps. Had to rewrite all of this because there is no fucking method
        // to iter over all fucking pairs of (key, value)

        for (key, values) in self.blizzards.iter_all() {
            for value in values.iter() {
                let (next_key, next_value) = match (key, value) {
                    ((row, col), Dir::South) if *row == self.rows-2 => ((1 as usize, *col), Dir::South),
                    ((row, col), Dir::South) => ((row+1, *col), Dir::South),

                    ((row, col), Dir::North) if *row == 1 => ((self.rows-2, *col), Dir::North),
                    ((row, col), Dir::North) => ((row-1, *col), Dir::North),

                    ((row, col), Dir::East) if *col == self.cols-2 => ((*row, 1 as usize), Dir::East),
                    ((row, col), Dir::East) => ((*row, col+1), Dir::East),

                    ((row, col), Dir::West) if *col == 1 => ((*row, self.cols-2), Dir::West),
                    ((row, col), Dir::West) => ((*row, col-1), Dir::West),
                };
                next_blizzards.insert(next_key, next_value);
            }
        }

        self.blizzards = next_blizzards;
    }

    fn is_accessible(&mut self, row: usize, col: usize, time: usize) -> bool {
        if time > self.snapshots.len() {
            panic!("time goes too fast");
        } else if time == self.snapshots.len() {
            self.advance_blizzards();
            self.take_snapshot();
        }

        self.snapshots[time][row][col] == '.'
    }

    fn draw_snapshot(&self, time: usize) {
        for row in self.snapshots[time].iter() {
            for col in row.iter() {
                print!("{}", *col);
            }
            println!("");
        }
        println!("");
    }
}

fn gen_tile_next(tile: &(usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let mut result = vec![
        (tile.0, tile.1, tile.2+1),
        (tile.0+1, tile.1, tile.2+1),
        (tile.0, tile.1+1, tile.2+1),
        (tile.0, tile.1-1, tile.2+1),
    ];

    if tile.0 > 0 {
        result.push((tile.0-1, tile.1, tile.2+1));
    }

    result
}

fn evaluate_shortest_path(map: &mut LazyMap, start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut open_set = vec![(start.0, start.1, 0)];
    let mut explored = std::collections::HashSet::<(usize, usize, usize)>::new();

    while ! open_set.is_empty() {
        let current = open_set.pop().unwrap();

        if (current.0, current.1) == goal {
            return current.2;
        }

        for next in gen_tile_next(&current).into_iter().filter(|t| map.is_accessible(t.0, t.1, t.2)) {
            if !explored.contains(&next) {
                open_set.push(next);
                explored.insert(next);
            }
        }

        open_set.sort_by(|a, b| {
            let a_to_goal = a.0.abs_diff(goal.0) + a.1.abs_diff(goal.1);
            let b_to_goal = b.0.abs_diff(goal.0) + b.1.abs_diff(goal.1);

            let whole_paths = (b_to_goal+b.2).cmp(&(a_to_goal+a.2));
            if whole_paths != std::cmp::Ordering::Equal {
                whole_paths
            } else {
                b.2.cmp(&a.2)
            }
        });
        open_set.dedup();
    }

    0
}


fn main() {
    // 328 was too high
    let text = std::fs::read_to_string("../input.txt").expect("cannot read input file");
    let (blizzards, size) = parse_input(&text);
    let mut map = LazyMap::new(blizzards, size);
    let start = (0, 1);
    let goal = (map.rows-1, map.cols-2);
    let shortest_path = evaluate_shortest_path(&mut map, start, goal);
    println!("shortest path: {}", shortest_path);
}

