use std::fs::read_to_string;
use std::collections::HashSet;

fn item_priority(item: &char) -> u32 {
    if item.is_lowercase() {
        return 1 + (*item as u32) - ('a' as u32);
    } else {
        return 27 + (*item as u32) - ('A' as u32);
    }
}

fn runsack_parts(content: Vec<char>) -> (HashSet<char>, HashSet<char>) {
    let mut parts_iter = content[..].chunks(content.len()/2);

    let left = HashSet::from_iter(parts_iter.next().unwrap().iter().cloned());
    let right = HashSet::from_iter(parts_iter.next().unwrap().iter().cloned());
    

    (left, right)
}

fn runsack_common(left: &HashSet<char>, right: &HashSet<char>) -> HashSet<char> {
    left.intersection(right).cloned().collect()
}

fn parse_input(content: String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in content.split("\n") {
        if line.is_empty() {
            continue;
        }

        let line_as_vec : Vec<char> = Vec::from(line.chars().collect::<Vec<char>>());
        result.push(line_as_vec);
    }

    result
}


fn main() {
    let input = parse_input(read_to_string("../input.txt").expect("failed to read input file"));
    let mut total: u32 = 0;

    for runsack in input {
        let (left, right) = runsack_parts(runsack);
        let common = runsack_common(&left, &right);
        total += common.iter().map(item_priority).sum::<u32>();
    }

    println!("total priority of runsacks: {}", total);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_item_priority() {
        use crate::item_priority;
        assert_eq!(item_priority(&'a'), 1);
        assert_eq!(item_priority(&'b'), 2);
        assert_eq!(item_priority(&'z'), 26);

        assert_eq!(item_priority(&'A'), 27);
        assert_eq!(item_priority(&'B'), 28);
        assert_eq!(item_priority(&'Z'), 52);
    }
}

