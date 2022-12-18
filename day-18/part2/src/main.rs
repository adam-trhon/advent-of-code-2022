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

fn gen_surface_area_cubes(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let mut surface_area_cubes: HashSet<Cube> = HashSet::new();

    for d in cubes {
        for x in -1..=1 as i8 {
            for y in -1..=1 as i8 {
                for z in -1..=1 as i8 {
                    if x==0 && y==0 && z==0 {
                        continue;
                    }
                    if !cubes.contains(&(d.0 + x, d.1 + y, d.2 + z)) {
                        surface_area_cubes.insert((d.0 + x, d.1 + y, d.2 + z));
                    }
                }
            }
        }
    }

    surface_area_cubes
}

fn gen_cube_groups(mut cubes: HashSet<Cube>) -> Vec<HashSet<Cube>> {
    let mut cube_groups: Vec<HashSet<Cube>> = Vec::new();

    while !cubes.is_empty() {
        let mut new_group = HashSet::<Cube>::new();
        let mut to_explore = HashSet::<Cube>::new();
        let new_group_seed = *cubes.iter().next().unwrap();

        new_group.insert(new_group_seed);
        to_explore.insert(new_group_seed);
        cubes.remove(&new_group_seed);

        loop {
            let mut to_explore_neighbours = HashSet::<Cube>::new();
            for cube in to_explore.iter() {
                to_explore_neighbours.extend(gen_6_neighbours(&cube).intersection(&cubes));
            }

            let mut newly_discovered = HashSet::<Cube>::new();
            newly_discovered.extend(to_explore_neighbours.difference(&new_group));

            if newly_discovered.is_empty() {
                break;
            }

            new_group.extend(newly_discovered.iter());
            to_explore = newly_discovered;

        }

        for c in new_group.iter() {
            cubes.remove(c);
        }
        cube_groups.push(new_group);
    }

    cube_groups
}

fn pick_surface_group(cube_groups: &Vec<HashSet<Cube>>) -> HashSet<Cube> {
    // bold move - we pick the biggest, although it might not be correct (i.e. geode)
    let mut biggest_len: usize = cube_groups[0].len();
    let mut biggest_index: usize = 0;

    for i in 1..cube_groups.len() {
        if cube_groups[i].len() > biggest_len {
            biggest_len = cube_groups[i].len();
            biggest_index = i;
        }
    }

    cube_groups[biggest_index].clone()
}

fn surface_area_of_group(droplets: &HashSet<Cube>, group: &HashSet<Cube>) -> u32 {
    let mut surface_area: u32 = 0;

    for droplet in droplets {
        surface_area += gen_6_neighbours(droplet).intersection(&group).count() as u32;
    }

    surface_area
}

fn count_surface_area(droplets: &HashSet<Cube>) -> u32 {

    let surface_area_cubes = gen_surface_area_cubes(&droplets);
    let surface_groups = gen_cube_groups(surface_area_cubes);
    let surface_group = pick_surface_group(&surface_groups);

    surface_area_of_group(droplets, &surface_group)
}

fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let droplets = parse_input(&text);
    println!("surface area: {}", count_surface_area(&droplets));
}
