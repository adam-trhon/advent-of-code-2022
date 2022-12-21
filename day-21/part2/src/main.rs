
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

    result.get_mut("humn").unwrap().result = None;

    result
}

fn update_monkey(monkeys: &mut MonkeyMap, name: &String) -> Option<i64> {
    let mut monkey: Monkey = monkeys[name].clone();

    if monkey.result.is_none() && name != "humn" {

        let left_name: String = monkey.left.clone().unwrap();
        let left_result = update_monkey(monkeys, &left_name);

        let op = monkey.op.unwrap();

        let right_name: String = monkey.right.clone().unwrap();
        let right_result = update_monkey(monkeys, &right_name);

        match (left_result, op, right_result) {
            (Some(left), '+', Some(right)) => monkey.result = Some(left + right),
            (Some(left), '-', Some(right)) => monkey.result = Some(left - right),
            (Some(left), '*', Some(right)) => monkey.result = Some(left * right),
            (Some(left), '/', Some(right)) => monkey.result = Some(left / right),
            _ => { }
        }
    }

    *monkeys.get_mut(name).unwrap() = monkey;
    monkeys[name].result.clone()
}

fn find_humn_value_from(monkeys: &MonkeyMap, name: &String, required: i64) -> i64 {
    if name == "humn" {
        return required;
    }

    let monkey: Monkey = monkeys[name].clone();
    let op = monkey.op.unwrap();

    let left_name: String = monkey.left.unwrap();
    let left_result = monkeys[&left_name].result.clone();
    let right_name: String = monkey.right.unwrap();
    let right_result = monkeys[&right_name].result.clone();

    match (left_result, op, right_result) {
        (Some(left), '+', None)        => find_humn_value_from(&monkeys, &right_name, required-left),
        (None,       '+', Some(right)) => find_humn_value_from(&monkeys, &left_name, required-right),
        (Some(left), '-', None)        => find_humn_value_from(&monkeys, &right_name, left-required),
        (None,       '-', Some(right)) => find_humn_value_from(&monkeys, &left_name, required+right),
        (Some(left), '*', None)        => find_humn_value_from(&monkeys, &right_name, required/left),
        (None,       '*', Some(right)) => find_humn_value_from(&monkeys, &left_name, required/right),
        (Some(left), '/', None)        => find_humn_value_from(&monkeys, &right_name, left/required),
        (None,       '/', Some(right)) => find_humn_value_from(&monkeys, &left_name, required*right),
        (Some(_), _, Some(_)) => panic!("invalid combination - both filled"),
        (None, _, None) => panic!("invalid combination - none filled"),
        _ => panic!("invalid operator"),
    }
}

fn find_humn_value(monkeys: &MonkeyMap) -> i64 {
    let root = monkeys["root"].clone();

    let left_name: String = root.right.unwrap();
    let left_result = monkeys[&left_name].result.clone();
    let right_name: String = root.left.unwrap();
    let right_result = monkeys[&right_name].result.clone();

    match (left_result, right_result) {
        (Some(left), None) => find_humn_value_from(monkeys, &right_name, left),
        (None, Some(right)) => find_humn_value_from(monkeys, &left_name, right),
        _ => panic!("invalid combination in root"),
    }
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let mut monkeys = parse_input(&text);
    update_monkey(&mut monkeys, &String::from("root"));
    println!("humn requires value {}", find_humn_value(&monkeys));

}
