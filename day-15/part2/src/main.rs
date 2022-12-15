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

fn get_max_x_forbidden_by_at_y(sensor: &Sensor, y: i32) -> i32 {
    let line_to_sensor = (sensor.position.y - y).abs();
    let beacon_to_sensor = distance(&sensor.position, &sensor.closest_beacon);

    if line_to_sensor > beacon_to_sensor {
        panic!("line {} not forbidden by sensor {:?}", y, sensor);
    }


    let max_x = sensor.position.x + beacon_to_sensor - line_to_sensor;

    /*
    println!("sensor: {:?}", sensor);
    println!("y: {}", y);
    println!("sensor position x: {}", sensor.position.x);
    println!("bacon to sensor: {}", beacon_to_sensor);
    println!("line_to_sensor: {}", line_to_sensor);
    println!("max_x: {}", max_x);
    */

    max_x
}

fn find_beacon_tuning_frequency(sensors: &Vec<Sensor>, max_coord: i32) -> i64 {
        for y in 0..=max_coord {
            println!("y: {}", y);
            let mut x: i32 = 0;
            'x_loop: while x <= max_coord {
                //println!("x: {}", x);
                for sensor in sensors {
                    if is_forbidden_by(&Coordinates{x:x, y:y}, &sensor) {
                        x = get_max_x_forbidden_by_at_y(&sensor, y) + 1;
                        continue 'x_loop;
                    }
                }
                return (x as i64)*4000000 + (y as i64);
            }
        }

        panic!("beacon not found");
}

fn main() {
    let input_text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let sensors = parse_input(&input_text);
    let tuning_frequency = find_beacon_tuning_frequency(&sensors, 4000000);
    println!("{}", tuning_frequency);
}
