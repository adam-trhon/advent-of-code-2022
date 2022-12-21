use std::collections::HashSet;

#[derive(Clone)]
struct Rock {
    filled: HashSet<(i32, i32)>,
}

impl Rock {
    fn new<const S: usize>(filled: [(i32, i32); S]) -> Rock {
        Rock{filled: HashSet::<(i32, i32)>::from(filled)}
    }

    fn moved(& self, dx: i32, dy: i32) -> Rock {
        Rock{ filled: self.filled.iter().map(|(x, y)| (*x+dx, *y+dy)).collect() }
    }

    fn has_x(&self, val: i32) -> bool {
        self.filled.iter().filter(|(x, _)| x == &val).next().is_some()
    }

    fn has_y(&self, val: i32) -> bool {
        self.filled.iter().filter(|(_, y)| y == &val).next().is_some()
    }
}

struct RockHeap {
    counter: usize,
    rocks: Vec<Rock>,
}

impl RockHeap {
    fn new() -> RockHeap {
        let mut result = RockHeap{counter: 0, rocks: Vec::new()};
        result.rocks.push(Rock::new([(0, 0), (1, 0), (2, 0), (3, 0)]));
        result.rocks.push(Rock::new([(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]));
        result.rocks.push(Rock::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]));
        result.rocks.push(Rock::new([(0, 0), (0, 1), (0, 2), (0, 3)]));
        result.rocks.push(Rock::new([(0, 0), (1, 0), (0, 1), (1, 1)]));
        result
    }

    fn release_rock(&mut self) -> Rock {
        let old_counter = self.counter;
        self.counter = self.counter + 1;
        self.rocks[old_counter%self.rocks.len()].clone()
    }
}

struct Input {
    values: Vec<i32>,
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
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let old_counter = self.counter;
        self.counter = (self.counter + 1) % self.values.len();
        Some(self.values[old_counter])
    }
}

struct Chamber {
    filled: HashSet<(i32, i32)>,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber { filled: HashSet::new() }
    }

    fn max_y(&self) -> i32 {
        self.filled.iter().map(|(_, y)| *y).max().unwrap_or(0)
    }

    fn intersects_with(&self, rock: &Rock) -> bool {
        rock.has_y(0) || self.filled.intersection(&rock.filled).next().is_some()
    }

    fn throw_in(&mut self, rock_heap: &mut RockHeap, input: &mut Input) {
        let mut rock = rock_heap.release_rock();
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

        let max_y = self.max_y();
        self.filled.retain(|(_, y)| y + 1000 > max_y);
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
    // Test gives 1514285714288 which is correct.
    // Input gives 1577207975413 is too low.
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let mut input = Input::new(&text);
    let mut chamber = Chamber::new();
    let mut rock_heap = RockHeap::new();

    let mut input_periods = HashSet::<usize>::new();

    let mut period_input_id: Option<usize> = None;
    let mut period_height: Option<i32> = None;
    let mut period_rock_count: Option<usize> = None;

    let mut total_height: u64 = 0;
    let mut total_rocks: u64 = 1_000_000_000_000;

    let ignore = HashSet::<usize>::from([
        // test:
        2,      // gives 1514285714289
        28,     // gives 1514285714288 // which is correct
        15,     // gives 1514285714288 // which is correct
        5,      // gives 1514285714288 // which is correct

        // input:
        1648, // gives 1577207977186
        1676, // gives 1577207977186
        1704, // gives 1577207977186
        1732, // gives 1577207977186
        1761, // gives 1577207977186

    ]);

    while (rock_heap.counter as u64) < total_rocks {
        let rock_id = rock_heap.counter % rock_heap.rocks.len();
        let input_id = input.counter % input.values.len();

        if total_height > 0 {
            // we are just simulating the rest
        } else if rock_id == 0 && period_input_id.is_none() && input.counter > 0 && !ignore.contains(&input_id) {
            if input_periods.contains(&input_id) {
                period_input_id = Some(input_id);
                period_height = Some(chamber.max_y());
                period_rock_count = Some(rock_heap.counter);
                println!("period point at input {} rock {} height {}", input_id, rock_heap.counter, chamber.max_y());
            } else {
                input_periods.insert(input_id);
            }
        } else if rock_id == 0 && period_input_id == Some(input_id) {
            println!("period point at input {} rock {} height {}", input_id, rock_heap.counter, chamber.max_y());
            let period_rocks: u64 = (rock_heap.counter-period_rock_count.unwrap()) as u64;
            let period_height_increase: u64 = (chamber.max_y()-period_height.unwrap() as i32) as u64;
            println!("period length {}", period_rocks);
            println!("period height increase: {}", period_height_increase);

            let rocks_remaining = total_rocks - rock_heap.counter as u64;
            println!("rocks remaining: {}", rocks_remaining);

            let periods_remaining = rocks_remaining / period_rocks;
            println!("periods remaining: {}", periods_remaining);

            let rocks_to_simulate = rocks_remaining % period_rocks;
            println!("rocks to simulate: {}", rocks_to_simulate);

            assert_eq!(periods_remaining*period_rocks + rocks_to_simulate, rocks_remaining);

            total_rocks = rock_heap.counter as u64 + rocks_to_simulate;

            total_height = periods_remaining * period_height_increase;
        }

        chamber.throw_in(&mut rock_heap, &mut input);
    }

    total_height += chamber.max_y() as u64;

    println!("max height in chamber: {}", total_height);
}
