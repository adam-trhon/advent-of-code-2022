use regex::Regex;

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    position: Coordinates,
    closest_beacon: Coordinates,
}


fn parse_input(text: &String) -> Vec<Sensor> {
    let mut result = Vec::<Sensor>::new();
    let regex_str = r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)";
    let re = Regex::new(regex_str).unwrap();

    for cap in re.captures_iter(text) {
        result.push(Sensor{
            position: Coordinates{x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap() },
            closest_beacon: Coordinates{x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap() },
        });
    }

    result
}

fn distance(l: &Coordinates, r: &Coordinates) -> i32 {
    (l.x-r.x).abs() + (l.y-r.y).abs()
}

fn is_forbidden_by(coord: &Coordinates, sensor: &Sensor) -> bool {
    distance(coord, &sensor.position) <= distance(&sensor.closest_beacon, &sensor.position)
}

fn min_covered_x(sensors: &Vec<Sensor>) -> i32 {
    let mut result = i32::MAX;
    for sensor in sensors {
        result = std::cmp::min(
            result,
            sensor.position.x - distance(&sensor.position, &sensor.closest_beacon)
        );
    }
    result
}

fn max_covered_x(sensors: &Vec<Sensor>) -> i32 {
    let mut result = i32::MIN;
    for sensor in sensors {
        result = std::cmp::max(
            result,
            sensor.position.x + distance(&sensor.position, &sensor.closest_beacon)
        );
    }
    result
}

fn count_impossible_positions(y: i32, sensors: &Vec<Sensor>) -> u32 {
    let mut result: u32 = 0;
    for x in min_covered_x(sensors)..=max_covered_x(sensors) {
        //print!("checking x = {}", x);
        let mut x_forbidden = false;
        for sensor in sensors {
            if sensor.position.x == x && sensor.position.y == y { // maybe?
                continue;
            }
            if sensor.closest_beacon.x == x && sensor.closest_beacon.y == y {
                continue;
            }
            if is_forbidden_by(&Coordinates{x: x, y: y}, sensor) {
                x_forbidden = true;
                break;
            }
        }
        if x_forbidden {
            result += 1;
        }
        //println!("  ... forbidden: {}", x_forbidden);
    }
    result
}

fn main() {
    let input_text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let sensors = parse_input(&input_text);
    let impossible_positions = count_impossible_positions(2000000, &sensors);
    println!("{}", impossible_positions);
}
