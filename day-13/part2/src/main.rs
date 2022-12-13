use std::cmp::Ordering;

#[derive(Clone)]
struct PacketData {
    number: Option<u32>,
    array: Vec<PacketData>,
}

impl std::fmt::Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.number {
            Some(n) => {
                write!(f, "{}", n).unwrap();
            }
            None => {
                _ = write!(f, "[").unwrap();
                if self.array.len() > 0 {
                    write!(f, "{}", self.array[0]).unwrap();
                }
                for i in 1..self.array.len() {
                    write!(f, ",{}", self.array[i]).unwrap();
                }
                write!(f, "]").unwrap();
            }
        };
        write!(f, "")
    }
}

fn parse_array(input: &Vec<char>, pos: usize) -> (Vec<PacketData>, usize) {
    let mut result = Vec::<PacketData>::new();
    let mut parsed: usize = 0;

    if input[pos] != '[' {
        panic!("array does not start at '[' but at '{}'", input[pos]);
    }
    parsed += 1;

    if input[pos+parsed] == ']' {
        parsed += 1;
        return (result, parsed)
    }

    loop {
            let (value, parsed_by_value) = parse_value(input, pos+parsed);
            result.push(value);
            parsed += parsed_by_value;

            if input[pos+parsed] == ',' {
                parsed += 1;
                continue;
            } else if input[pos+parsed] == ']' {
                parsed += 1;
                break;
            } else {
                panic!("invalid character after value: {}", input[pos+parsed]);
            }
    }

    (result, parsed)
}

fn parse_value(input: &Vec<char>, pos: usize) -> (PacketData, usize) {
    let mut result = PacketData{number: None, array: Vec::new()};
    let mut parsed: usize = 0;

    if input[pos].is_digit(10) {
        let mut result_number: u32 = 0;
        while input[pos+parsed].is_digit(10) {
            result_number *= 10;
            result_number += (input[pos+parsed] as u32) - ('0' as u32);
            parsed += 1;
        }
        result.number = Some(result_number);
    } else if input[pos] == '[' {
        let (value_array, parsed_chars) = parse_array(input, pos);
        result.array = value_array;
        parsed = parsed_chars;
    } else {
        panic!("cannot parse value, unexpected character {}", input[pos]);
    }

    (result, parsed)
}

fn parse_input(input: &String) -> Vec<PacketData> {
    let mut result: Vec<PacketData> = Vec::new();

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        result.push(parse_value(&line.chars().collect(), 0).0);
    }

    result
}

fn divider_packet(value: u32) -> PacketData {
    let mut divider_packet = PacketData{number: None, array: Vec::new()};
    divider_packet.array.push(divider_packet.clone());
    divider_packet.array[0].array.push(PacketData{number: Some(value), array: Vec::new()});
    divider_packet
}

fn compare(left: &PacketData, right: &PacketData) -> Ordering {
    if left.number.is_some() && right.number.is_none() {
        let left_array = PacketData{number: None, array: Vec::from([left.clone(); 1])};
        return compare(&left_array, right);
    } else if left.number.is_none() && right.number.is_some() {
        let right_array = PacketData{number: None, array: Vec::from([right.clone(); 1])};
        return compare(left, &right_array);
    } else if left.number.is_some() && right.number.is_some() {
        if left.number.unwrap() < right.number.unwrap() {
            return Ordering::Less;
        } else if left.number.unwrap() > right.number.unwrap() {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    } else {
        let mut index: usize = 0;
        loop {
            if left.array.len() == index && right.array.len() == index {
                return Ordering::Equal;
            }
            if left.array.len() == index && right.array.len() > index {
                return Ordering::Less;
            }
            if left.array.len() > index && right.array.len() == index {
                return Ordering::Greater;
            }
            let item_comparison = compare(&left.array[index], &right.array[index]);
            if item_comparison != Ordering::Equal {
                return item_comparison;
            }
            index += 1;
        }
    }
}

fn evaluate(data: &Vec<PacketData>) -> u32 {
    let mut value: u32 = 1;

    value *= data.iter().position(|d| compare(d, &divider_packet(2)) == Ordering::Equal)
        .unwrap() as u32 + 1;
    value *= data.iter().position(|d| compare(d, &divider_packet(6)) == Ordering::Equal)
        .unwrap() as u32 + 1;

    value
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let mut input = parse_input(&text);
    input.push(divider_packet(2));
    input.push(divider_packet(6));
    input.sort_by(compare);
    println!("value: {}", evaluate(&input));

}
