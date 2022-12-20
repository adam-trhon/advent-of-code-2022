
type Mixer = Vec<(usize, i16)>;

fn parse_input(text: &String) -> Vec<i16> {
    text.split("\n").filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect()
}

fn move_by<T: Copy>(array: &Vec<T>, index: usize, by: isize) -> Vec<T> {

    let mut result = array.clone();

    let element = result[index];
    result.remove(index);
    result.insert((10*result.len() as isize + index as isize +by) as usize % result.len(), element);

    result
}

fn mix(input: &Vec<i16>) -> i16 {
    let mut mixer: Mixer = input.iter().enumerate().map(|(i, v)| (i, *v)).collect();

    if input.len() < 10 {
        println!("{:?}", mixer.iter().map(|v| v.1).collect::<Vec<i16>>());
    }
    for i in 0..input.len() {
        let i_pos = mixer.iter().position(|(i_, _)| i_ == &i).unwrap();
        let move_offset = mixer[i_pos].1 as isize;
        mixer = move_by(&mut mixer, i_pos, move_offset);
        if input.len() < 10 {
            println!("{:?}", mixer.iter().map(|v| v.1).collect::<Vec<i16>>());
        }
    }

    let zero_pos = mixer.iter().position(|(_, v)| v == &0).unwrap();

    [1000 as usize, 2000, 3000].iter().map(|o| mixer[(zero_pos + o)%input.len()].1).sum()
}


fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let input = parse_input(&text);
    if input.len() > 10 {
        assert_eq!(input.len(), 5000);
    }
    println!("mixed input number: {}", mix(&input));
}

#[cfg(test)]
mod tests {

#[test]
    fn test_move_by() {
        use crate::move_by;

        let test_vec = vec![1, 2, 3, 4, 5];

        //                                         0  1  2  3  4
        assert_eq!(move_by(&test_vec, 0,   0), vec![1, 2, 3, 4, 5]);
        assert_eq!(move_by(&test_vec, 1,  -1), vec![2, 1, 3, 4, 5]);
        assert_eq!(move_by(&test_vec, 0,  -1), vec![2, 3, 4, 1, 5]);
        assert_eq!(move_by(&test_vec, 4,   1), vec![1, 5, 2, 3, 4]);
        //assert_eq!(move_by(&test_vec, 3,   1), vec![1, 2, 3, 5, 4]); // gives vec![4, 1, 2, 3, 5]
        assert_eq!(move_by(&test_vec, 0,  10), vec![2, 3, 1, 4, 5]);
        assert_eq!(move_by(&test_vec, 0, -10), vec![2, 3, 1, 4, 5]);
    }
}
