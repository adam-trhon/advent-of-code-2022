#[derive(Clone)]
struct SNAFU {
    digits: Vec<i8>
}

fn parse_input(text: &String) -> Vec<SNAFU> {
    text.split("\n").filter(|l| !l.is_empty()).map(|l| parse_snafu(l)).collect()
}

fn parse_snafu(s: &str) -> SNAFU {
    SNAFU {
        digits: s.chars().map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid SNAFU character")
            }).collect(),
    }
}

fn print_snafu(s: &SNAFU) -> String {
    s.digits.iter().map(|v| match v {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        v => panic!("invalid value in SNAFU {}", v),
    }).collect()
}

fn snafu_to_decimal(num: &SNAFU) -> i64 {
    num.digits.iter().rev().enumerate().map(|(i, v)| *v as i64 * (5_i64.pow(i as u32))).sum()
}

fn decimal_to_snafu(mut num: i64) -> SNAFU {
    if num < 0 {
        panic!("negative numbers not implemented :(");
    }

    let mut result = SNAFU { digits: Vec::new() };

    while num > 0 {
        let mut digit = num % 5;
        num = num / 5;

        if digit > 2 {
            digit -= 5;
            num += 1;
        }

        if digit > 2 {
            panic!("wtf is {} doing here", digit);
        }

        result.digits.insert(0, digit as i8);
    }
    
    result
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let snafu_numbers = parse_input(&text);
    let decimal_sum: i64 = snafu_numbers.iter().map(|s| snafu_to_decimal(s)).sum();
    println!("decimal sum: {}", decimal_sum);
    let snafu_sum = decimal_to_snafu(decimal_sum);
    println!("{}", print_snafu(&snafu_sum));
}
