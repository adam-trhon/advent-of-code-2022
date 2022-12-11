
use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    operation_mult: u64,
    operation_add: u64,
    operation_pow: u32,
    test_divisible: u64,
    target_true: usize,
    target_false: usize,
    inspected: u64,
}

fn parse_input(text: &String) -> Vec<Monkey> {
    let mut result = Vec::<Monkey>::new();

    for monkey_data in text.split("\n").collect::<Vec<&str>>().chunks(7) {
        if !monkey_data[0].starts_with("Monkey") {
            println!("monkey does not start with 'Monkey'");
            continue;
        }
        let items_str = monkey_data[1].split(":").nth(1).unwrap();
        let items: VecDeque<u64> = items_str.split(",").map(|s| s.trim().parse::<u64>().unwrap()).collect();

        let operation_mult: u64;
        let operation_add: u64;
        let operation_pow: u32;

        if monkey_data[2].contains("*") {
            let multiplied_by = monkey_data[2].split("*").nth(1).unwrap().trim();
            if multiplied_by == "old" {
                operation_mult = 1;
                operation_pow = 2;
            } else {
                operation_mult = multiplied_by.parse().unwrap();
                operation_pow = 1;
            }
            operation_add = 0;
        } else if monkey_data[2].contains("+") {
            operation_mult = 1;
            operation_pow = 1;
            operation_add = monkey_data[2].split("+").nth(1).unwrap().trim().parse().unwrap();
        } else {
            println!("invalid operation line");
            continue;
        }

        let test_divisible: u64;
        if monkey_data[3].contains("Test") {
            test_divisible = monkey_data[3].split(" ").last().unwrap().parse().unwrap();
        } else {
            println!("invalid test line");
            continue;
        }

        let target_true: usize;
        if monkey_data[4].contains("true") {
            target_true = monkey_data[4].split(" ").last().unwrap().parse().unwrap();
        } else {
            println!("invalid target true line");
            continue;
        }

        let target_false: usize;
        if monkey_data[5].contains("false") {
            target_false = monkey_data[5].split(" ").last().unwrap().parse().unwrap();
        } else {
            println!("invalid target false line");
            continue
        }

        result.push(Monkey{
            items: items,
            operation_mult: operation_mult,
            operation_pow: operation_pow,
            operation_add: operation_add,
            test_divisible: test_divisible,
            target_true: target_true,
            target_false: target_false,
            inspected: 0,
        });
    }

    result
}

fn evaluate_item(monkeys: &mut Vec<Monkey>, chosen: usize, modulus: u64) {
    let item = monkeys[chosen].items.pop_front().unwrap();
    let mut item_eval = item;
    item_eval *= monkeys[chosen].operation_mult;
    item_eval += monkeys[chosen].operation_add;
    item_eval = item_eval.pow(monkeys[chosen].operation_pow);
    item_eval = item_eval % modulus;

    let target: usize;
    if item_eval % monkeys[chosen].test_divisible == 0 {
        target = monkeys[chosen].target_true;
    } else {
        target = monkeys[chosen].target_false;
    }

    monkeys[target].items.push_back(item_eval);
    monkeys[chosen].inspected += 1;
}

fn evaluate_turn(monkeys: &mut Vec<Monkey>, chosen: usize, modulus: u64) {
    while monkeys[chosen].items.len() > 0 {
        evaluate_item(monkeys, chosen, modulus);
    }
}

fn evaluate_round(monkeys: &mut Vec<Monkey>, modulus: u64) {
    let monkey_count = monkeys.len();

    for current_monkey in 0..monkey_count {
        evaluate_turn(monkeys, current_monkey, modulus);
    }
}

fn compute_monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    inspected.sort();
    inspected.reverse();
    inspected[0] * inspected[1]
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let mut monkeys = parse_input(&input);
    let modulus: u64 = monkeys.iter().map(|m| m.test_divisible).product();
    println!("modulus: {}", modulus);
    for i in 0..10000 {
        evaluate_round(&mut monkeys, modulus);
    }

    println!("Monkey business value: {}", compute_monkey_business(&monkeys));
}
