use std::fs::read_to_string;
use std::collections::HashSet;

fn item_priority(item: &char) -> u32 {
    if item.is_lowercase() {
        return 1 + (*item as u32) - ('a' as u32);
    } else {
        return 27 + (*item as u32) - ('A' as u32);
    }
}

fn find_badge(runsack_tripple: &[Vec<char>]) -> char {
    let r0 = HashSet::<char>::from_iter(runsack_tripple[0].iter().cloned());
    let r1 = HashSet::<char>::from_iter(runsack_tripple[1].iter().cloned());
    let r2 = HashSet::<char>::from_iter(runsack_tripple[2].iter().cloned());

    let r0_r1_common: HashSet<char> = r0.intersection(&r1).cloned().collect();
    let badge_set: HashSet<char> = r0_r1_common.intersection(&r2).cloned().collect();

    *badge_set.iter().next().unwrap()
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

    
    for runsack_tripple in input.chunks(3) {
        let badge = find_badge(runsack_tripple);
        total += item_priority(&badge);
    }

    println!("total priority of badges: {}", total);
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

