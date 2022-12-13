
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

fn parse_input(input: &String) -> Vec<(PacketData, PacketData)> {
    let mut result: Vec<(PacketData, PacketData)> = Vec::new();

    for chunk in input.split("\n").collect::<Vec<&str>>().chunks(3) {
        let (value1, _) = parse_value(&chunk[0].chars().collect(), 0);
        let (value2, _) = parse_value(&chunk[1].chars().collect(), 0);
        result.push((value1, value2));
    }

    result
}

fn compare(left: &PacketData, right: &PacketData) -> i32 {
    if left.number.is_some() && right.number.is_none() {
        let left_array = PacketData{number: None, array: Vec::from([left.clone(); 1])};
        return compare(&left_array, right);
    } else if left.number.is_none() && right.number.is_some() {
        let right_array = PacketData{number: None, array: Vec::from([right.clone(); 1])};
        return compare(left, &right_array);
    } else if left.number.is_some() && right.number.is_some() {
        if left.number.unwrap() < right.number.unwrap() {
            return -1;
        } else if left.number.unwrap() > right.number.unwrap() {
            return 1;
        } else {
            return 0;
        }
    } else {
        let mut index: usize = 0;
        loop {
            if left.array.len() == index && right.array.len() == index {
                return 0;
            }
            if left.array.len() == index && right.array.len() > index {
                return -1;
            }
            if left.array.len() > index && right.array.len() == index {
                return 1;
            }
            let item_comparison = compare(&left.array[index], &right.array[index]);
            if item_comparison != 0 {
                return item_comparison;
            }
            index += 1;
        }
    }
}

fn get_input_score(input: &Vec<(PacketData, PacketData)>) -> u32 {
    let mut score: u32 = 0;
    for (index, (left, right)) in input.iter().enumerate() {
        if compare(&left, &right) == -1 {
            score += index as u32 + 1;
        }
    }
    score
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let input = parse_input(&text);
    println!("score: {}", get_input_score(&input));
}
