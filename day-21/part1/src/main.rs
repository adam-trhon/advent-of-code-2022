
#[derive(Clone)]
struct Monkey {
    left: Option<String>,
    op: Option<char>,
    right: Option<String>,
    result: Option<i64>,
}

type MonkeyMap = std::collections::HashMap<String, Monkey>;

fn parse_input(text: &String) -> MonkeyMap {
    let mut result = MonkeyMap::new();

    let number_monkey_re = regex::Regex::new(r"(....): (-?\d+)").unwrap();
    let operation_monkey_re = regex::Regex::new(r"(....): (....) (.) (....)").unwrap();

    result.extend(number_monkey_re.captures_iter(text).map(|c| {
        let name = String::from(&c[1]);
        let result = c[2].parse().unwrap();
        (name, Monkey { left: None, op: None, right: None, result: Some(result)})
    }));

    result.extend(operation_monkey_re.captures_iter(text).map(|c| {
        let name = String::from(&c[1]);
        let left = String::from(&c[2]);
        let op = c[3].chars().next().unwrap();
        let right = String::from(&c[4]);
        (name, Monkey {left: Some(left), op: Some(op), right: Some(right), result: None})
    }));

    result
}

fn compute_monkey(monkeys: &MonkeyMap, name: &String) -> i64 {
    let mut monkey: Monkey = monkeys[name].clone();

    if monkey.result.is_none() {

        let left_name: String = monkey.left.clone().unwrap();
        let left = compute_monkey(monkeys, &left_name);

        let op = monkey.op.unwrap();

        let right_name: String = monkey.right.clone().unwrap();
        let right = compute_monkey(monkeys, &right_name);

        match op {
            '+' => monkey.result = Some(left + right),
            '-' => monkey.result = Some(left - right),
            '*' => monkey.result = Some(left * right),
            '/' => monkey.result = Some(left / right),
            _ => panic!("unknown operation"),
        }
    }

    monkey.result.unwrap()
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let monkeys = parse_input(&text);
    println!("root value: {}", compute_monkey(&monkeys, &String::from("root")));
}
