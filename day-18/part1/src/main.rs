use std::collections::HashSet;

type Cube = (i8, i8, i8);

fn parse_input(text: &String) -> HashSet<Cube> {
    let mut result: HashSet<Cube> = HashSet::new();

    for line in text.split("\n") {
        if line.is_empty() {
            continue;
        }
        let  line_vec: Vec<i8> = line.split(",").map(|s| s.parse().unwrap()).collect();
        match line_vec[..] {
            [a, b, c] => result.insert((a, b, c)),
            _ => panic!("invalid line")
        };
    }

    result

}

fn gen_6_neighbours(cube: &Cube) -> HashSet<Cube> {
    HashSet::from([
        ((cube.0+1, cube.1,   cube.2)),
        ((cube.0-1, cube.1,   cube.2)),
        ((cube.0,   cube.1+1, cube.2)),
        ((cube.0,   cube.1-1, cube.2)),
        ((cube.0,   cube.1,   cube.2+1)),
        ((cube.0,   cube.1,   cube.2-1)),
    ])
}

fn count_surface_area(droplets: &HashSet<Cube>) -> u32 {
    let mut surface_area: u32 = 0;

    for droplet in droplets {
        surface_area += gen_6_neighbours(droplet).difference(&droplets).count() as u32;
    }

    surface_area
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let droplets = parse_input(&text);
    println!("surface area: {}", count_surface_area(&droplets));
}
