
const VOID: u8 = 0;
const TILE: u8 = 1;
const WALL: u8 = 2;

type Row = Vec<u8>;
type Map = Vec<Row>;

#[derive(Debug,Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

type Pos = (usize, usize, Direction);

#[derive(Debug,Clone)]
enum Instruction {
    Left,
    Right,
    Walk(usize),
}
type Instructions = Vec<Instruction>;

fn parse_input(text: &String) -> (Map, Instructions) {
    let mut map = Map::new();
    let instructions: Instructions;

    let mut lines = text.split("\n");

    loop {
        match lines.next() {
            Some("") => break,
            Some(line) => map.push(line_to_row(&line)),
            None => panic!("unexpected eof"),
        }
    }

    instructions = line_to_instructions(lines.next().unwrap());

    (map, instructions)
}

fn line_to_row(line: &str) -> Row {
    line.chars().map(|c| match c {
        ' ' => VOID,
        '.' => TILE,
        '#' => WALL,
        _ => panic!("invalid charater"),
    }).collect()
}

fn line_to_instructions(line: &str) -> Instructions {
    lazy_static::lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(L|R|\d+)").unwrap();
    }
    RE.captures_iter(line).map(|c| match &c[0] {
        "L" => Instruction::Left,
        "R" => Instruction::Right,
        v => Instruction::Walk(v.parse().unwrap()),
    }).collect()
}

fn get_initial_position(map: &Map) -> Pos {
    (0, map[0].iter().position(|&e| e == TILE).unwrap(), Direction::Right)
}

fn follow(p: Pos, i: &Instruction, map: &Map) -> Pos {
    match (i, &p.2) {
        (Instruction::Left, Direction::Left) => (p.0, p.1, Direction::Down),
        (Instruction::Left, Direction::Up) => (p.0, p.1, Direction::Left),
        (Instruction::Left, Direction::Right) => (p.0, p.1, Direction::Up),
        (Instruction::Left, Direction::Down) => (p.0, p.1, Direction::Right),

        (Instruction::Right, Direction::Left) => (p.0, p.1, Direction::Up),
        (Instruction::Right, Direction::Up) => (p.0, p.1, Direction::Right),
        (Instruction::Right, Direction::Right) => (p.0, p.1, Direction::Down),
        (Instruction::Right, Direction::Down) => (p.0, p.1, Direction::Left),

        (Instruction::Walk(d), Direction::Left) => walk_left(p, &map, *d),
        (Instruction::Walk(d), Direction::Up) => walk_up(p, &map, *d),
        (Instruction::Walk(d), Direction::Right) => walk_right(p, &map, *d),
        (Instruction::Walk(d), Direction::Down) => walk_down(p, &map, *d),
    }
}

fn walk(p: Pos, map: &Map, distance: usize) -> Pos {
    match p.2 {
        Direction::Left => walk_left(p, map, distance),
        Direction::Up => walk_up(p, map, distance),
        Direction::Down => walk_down(p, map, distance),
        Direction::Right => walk_right(p, map, distance),
    }
}

fn walk_left(mut p: Pos, map: &Map, distance: usize) -> Pos {
    for i in 0..distance {
        if p.1 == 0 || map[p.0][p.1-1] == VOID {
            p = wrap_left(p, map, distance-i);
        } else if map[p.0][p.1-1] == TILE {
            p.1 = p.1 - 1;
        } else if map[p.0][p.1-1] == WALL {
            break;
        }
    }

    p
}

fn wrap_left(p: Pos, map: &Map, distance: usize) -> Pos {
    let new_pos: Pos;
    if p.0 < 4 {
        new_pos = (4, 4+p.1, Direction::Down);
    } else if p.0 < 8 {
        new_pos = (11, 15-(p.0-4), Direction::Up);
    } else if p.0 < 12 {
        new_pos = (7, 8-(p.0-8), Direction::Up); 
    } else {
        panic!("cannot wrap left");
    }

    if map[new_pos.0][new_pos.1] == WALL {
        p
    } else {
        walk(new_pos, map, distance-1)
    }
}

fn walk_right(mut p: Pos, map: &Map, distance: usize) -> Pos {
    for i in 0..distance {
        if p.1+1 == map[p.0].len() || map[p.0][p.1+1] == VOID {
            p = wrap_right(p, map, distance-i);
            break;
        } else if map[p.0][p.1+1] == TILE {
            p.1 = p.1 + 1;
        } else if map[p.0][p.1+1] == WALL {
            break;
        }
    }

    p
}

fn wrap_right(p: Pos, map: &Map, distance: usize) -> Pos {
    let new_pos: Pos;

    if p.0 < 4 {
        new_pos = (11-p.0, 15, Direction::Left);
    } else if p.0 < 8 {
        new_pos = (8, 15-(p.0-4), Direction::Down);
    } else if p.0 < 12 {
        new_pos = (3-(p.0-8), 11, Direction::Left); 
    } else {
        panic!("cannot wrap right");
    }

    if map[new_pos.0][new_pos.1] == WALL {
        p
    } else {
        walk(new_pos, map, distance-1)
    }
}

fn walk_up(mut p: Pos, map: &Map, distance: usize) -> Pos {
    for i in 0..distance {
        if p.0 == 0 || map[p.0-1].len() <= p.1 || map[p.0-1][p.1] == VOID {
            p = wrap_up(p, map, distance-i);
            break;
        } else if map[p.0-1][p.1] == TILE {
            p.0 = p.0 - 1;
        } else if map[p.0-1][p.1] == WALL {
            break;
        }
    }

    p
}

fn wrap_up(p: Pos, map: &Map, distance: usize) -> Pos {
    let new_pos: Pos;

    if p.1 < 4 {
        new_pos = (0, 11-p.1, Direction::Down);
    } else if p.1 < 8 {
        new_pos = (p.1-4, 8, Direction::Right);
    } else if p.1 < 12 {
        new_pos = (5, 3-(p.1-8), Direction::Down); 
    } else if p.1 < 15 {
        new_pos = (7-(p.1-12), 11, Direction::Left); 
    } else {
        panic!("cannot wrap up");
    }

    if map[new_pos.0][new_pos.1] == WALL {
        p
    } else {
        walk(new_pos, map, distance-1)
    }
}

fn walk_down(mut p: Pos, map: &Map, distance: usize) -> Pos {
    for i in 0..distance {
        if p.0+1 == map.len() || map[p.0+1].len() <= p.1 || map[p.0+1][p.1] == VOID {
            p = wrap_down(p, map, distance-i);
            break;
        } else if map[p.0+1][p.1] == TILE {
            p.0 = p.0 + 1;
        } else if map[p.0+1][p.1] == WALL {
            break;
        }
    }

    p
}

fn wrap_down(p: Pos, map: &Map, distance: usize) -> Pos {
    let new_pos: Pos;

    if p.1 < 4 {
        new_pos = (11, 11-p.1, Direction::Up);
    } else if p.1 < 8 {
        new_pos = (11-p.1, 8, Direction::Right);
    } else if p.1 < 12 {
        new_pos = (7, 3-(p.1-8), Direction::Up); 
    } else if p.1 < 15 {
        new_pos = (7-(p.1-12), 0, Direction::Right); 
    } else {
        panic!("cannot wrap up");
    }

    if map[new_pos.0][new_pos.1] == WALL {
        p
    } else {
        walk(new_pos, map, distance-1)
    }
}

fn eval_direction(d: &Direction) -> usize {
    match d {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

fn draw(map: &Map, pos: &Pos) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if pos.0 == row && pos.1 == col {
                match pos.2 {
                    Direction::Right => print!(">"),
                    Direction::Up => print!("^"),
                    Direction::Left => print!("<"),
                    Direction::Down => print!("v"),
                }
            } else {
                match map[row][col] {
                    TILE => print!("."),
                    WALL => print!("#"),
                    _ => print!(" "),
                }
            }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let text = std::fs::read_to_string("../test.txt").expect("failed to read input file");
    let (map, instructions) = parse_input(&text);
    let mut pos = get_initial_position(&map);
    draw(&map, &pos);
    for i in instructions {
        println!("{:?}", i);
        pos = follow(pos, &i, &map);
        draw(&map, &pos);
    }
    let result = (pos.0+1)*1000 + (pos.1+1)*4 + eval_direction(&pos.2);
    println!("{}", result);

}
