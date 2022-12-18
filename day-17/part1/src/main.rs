use std::collections::HashSet;

#[derive(Clone)]
struct Rock {
    filled: HashSet<(i16, i16)>,
}

impl Rock {
    fn new<const S: usize>(filled: [(i16, i16); S]) -> Rock {
        Rock{filled: HashSet::<(i16, i16)>::from(filled)}
    }

    fn moved(& self, dx: i16, dy: i16) -> Rock {
        Rock{ filled: self.filled.iter().map(|(x, y)| (*x+dx, *y+dy)).collect() }
    }

    fn has_x(&self, val: i16) -> bool {
        self.filled.iter().filter(|(x, _)| x == &val).next().is_some()
    }

    fn has_y(&self, val: i16) -> bool {
        self.filled.iter().filter(|(_, y)| y == &val).next().is_some()
    }
}

struct RockGenerator {
    counter: usize,
    count: usize,
    rocks: Vec<Rock>,
}

impl RockGenerator {
    fn new(count: usize) -> RockGenerator {
        let mut result = RockGenerator{count: count, counter: 0, rocks: Vec::new()};
        result.rocks.push(Rock::new([(0, 0), (1, 0), (2, 0), (3, 0)]));
        result.rocks.push(Rock::new([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]));
        result.rocks.push(Rock::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]));
        result.rocks.push(Rock::new([(0, 0), (0, 1), (0, 2), (0, 3)]));
        result.rocks.push(Rock::new([(0, 0), (1, 0), (0, 1), (1, 1)]));
        result
    }
}

impl Iterator for RockGenerator {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            let old_counter = self.counter;
            self.counter = (self.counter + 1) % self.rocks.len();
            Some(self.rocks[old_counter].clone())
        }
    }
}

struct Input {
    values: Vec<i16>,
    counter: usize,
}

impl Input {
    fn new(text: &String) -> Input {
        Input{
            counter: 0,
            values: text.chars()
                .filter(|c| "<>".contains(*c))
                .map(|c| match c { '<' => -1, '>' => 1, _ => panic!("invalid char")})
                .collect(),
        }
    }
}

impl Iterator for Input {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        let old_counter = self.counter;
        self.counter = (self.counter + 1) % self.values.len();
        Some(self.values[old_counter])
    }
}

struct Chamber {
    filled: HashSet<(i16, i16)>,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber { filled: HashSet::new() }
    }

    fn max_y(&self) -> i16 {
        self.filled.iter().map(|(_, y)| *y).max().unwrap_or(0)
    }

    fn intersects_with(&self, rock: &Rock) -> bool {
        rock.has_y(0) || self.filled.intersection(&rock.filled).next().is_some()
    }

    fn throw_in(&mut self, mut rock: Rock, input: &mut Input) {
        rock = rock.moved(3, self.max_y() + 4);
        //self.draw(Some(&rock));

        loop {
            let shifted_rock = rock.moved(input.next().unwrap(), 0);
            if !shifted_rock.has_x(0) && !shifted_rock.has_x(8) && !self.intersects_with(&shifted_rock) {
                rock = shifted_rock;
            }

            let fallen_rock = rock.moved(0, -1);
            if self.intersects_with(&fallen_rock) {
                self.filled.extend(rock.filled.into_iter());
                break;
            } else {
                rock = fallen_rock;
            }
        }
    }

    #[allow(dead_code)]
    fn draw(&self, rock: Option<&Rock>) {
        let max_y = self.max_y();
        let mut drawing: Vec<Vec<char>> = Vec::new();
        drawing.push(vec!['+', '-', '-', '-', '-', '-', '-', '-', '+']);
        for _ in 1..max_y + 8 {
            drawing.push(vec!['|', '.', '.', '.', '.', '.', '.', '.', '|']);
        }
        for tile in self.filled.iter() {
            drawing[tile.1 as usize][tile.0 as usize] = '#';
        }
        if rock.is_some() {
            for tile in rock.unwrap().filled.iter() {
                drawing[tile.1 as usize][tile.0 as usize] = '@';
            }
        }

        println!("");
        for line in drawing.iter().rev() {
            for c in line {
                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let mut input = Input::new(&text);
    let mut chamber = Chamber::new();
    let rock_generator = RockGenerator::new(2022);
    for rock in rock_generator {
        chamber.throw_in(rock, &mut input);
    }
    println!("max height in chamber: {}", chamber.max_y());
}
