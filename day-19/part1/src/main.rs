#[derive(Copy,Clone,Debug)]
struct State {
    blueprint: u32,
    time_left: u32,
    ore_robot_ore: u32,
    ore_robot_count: u32,
    ore_count: u32,
    clay_robot_ore: u32,
    clay_robot_count: u32,
    clay_count: u32,
    obsidian_robot_ore: u32,
    obsidian_robot_clay: u32,
    obsidian_robot_count: u32,
    obsidian_count: u32,
    geode_robot_ore: u32,
    geode_robot_obsidian: u32,
    geode_robot_count: u32,
    geode_count: u32,
}

fn parse_input(text: &String) -> Vec<State> {
    let plan_re = regex::Regex::new(r"(\d+).* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) ").unwrap();

    plan_re.captures_iter(text).map(|capture| State {
        blueprint: capture[1].parse().unwrap(),
        time_left: 24,
        ore_robot_ore: capture[2].parse().unwrap(),
        ore_robot_count: 1,
        ore_count: 0,
        clay_robot_ore: capture[3].parse().unwrap(),
        clay_robot_count: 0,
        clay_count: 0,
        obsidian_robot_ore: capture[4].parse().unwrap(),
        obsidian_robot_clay: capture[5].parse().unwrap(),
        obsidian_robot_count: 0,
        obsidian_count: 0,
        geode_robot_ore: capture[6].parse().unwrap(),
        geode_robot_obsidian: capture[7].parse().unwrap(),
        geode_robot_count: 0,
        geode_count: 0,
    }).collect()
}

fn advance_state(state: &mut State) {
    state.time_left -= 1;
    state.ore_count += state.ore_robot_count;
    state.clay_count += state.clay_robot_count;
    state.obsidian_count += state.obsidian_robot_count;
    state.geode_count += state.geode_robot_count;
}

fn step_make_ore_robot(state: &mut State) {
    while state.ore_count < state.ore_robot_ore && state.time_left > 0 {
        advance_state(state);
    }
    if state.time_left > 0 {
        advance_state(state);
        state.ore_count -= state.ore_robot_ore;
        state.ore_robot_count += 1;
    }
}

fn step_make_clay_robot(state: &mut State) {
    while state.ore_count < state.clay_robot_ore && state.time_left > 0 {
        advance_state(state);
    }
    if state.time_left > 0 {
        advance_state(state);
        state.ore_count -= state.clay_robot_ore;
        state.clay_robot_count += 1;
    }
}

fn step_make_obsidian_robot(state: &mut State) {
    while (state.ore_count < state.obsidian_robot_ore ||
       state.clay_count < state.obsidian_robot_clay) && state.time_left > 0 {
        advance_state(state)
    }
    if state.time_left > 0 {
        advance_state(state);
        state.ore_count -= state.obsidian_robot_ore;
        state.clay_count -= state.obsidian_robot_clay;
        state.obsidian_robot_count += 1;
    }
}

fn step_make_geode_robot(state: &mut State) {
    while (state.ore_count < state.geode_robot_ore ||
       state.obsidian_count < state.geode_robot_obsidian) && state.time_left > 0 {
        advance_state(state);
    }
    if state.time_left > 0 {
        advance_state(state);
        state.ore_count -= state.geode_robot_ore;
        state.obsidian_count -= state.geode_robot_obsidian;
        state.geode_robot_count += 1;
    }
}

fn evaluate_plan(state: State) -> u32 {
    if state.time_left == 0 {
        return state.geode_count;
    }

    let mut best_plan: u32 = 0;

    let mut ore_next = state.clone();
    step_make_ore_robot(&mut ore_next);
    best_plan = std::cmp::max(best_plan, evaluate_plan(ore_next));

    let mut clay_next = state.clone();
    step_make_clay_robot(&mut clay_next);
    best_plan = std::cmp::max(best_plan, evaluate_plan(clay_next));

    let mut obsidian_next = state.clone();
    step_make_obsidian_robot(&mut obsidian_next);
    best_plan = std::cmp::max(best_plan, evaluate_plan(obsidian_next));

    let mut geode_next = state.clone();
    step_make_geode_robot(&mut geode_next);
    best_plan = std::cmp::max(best_plan, evaluate_plan(geode_next));

    best_plan
}

fn main() {
    // 1267 is not correct
    let input = std::fs::read_to_string("../input.txt").expect("failed to read input file");
    let states = parse_input(&input);

    let r: u32 = states.iter().map(|state| {
        println!("evaluating {}", state.blueprint);
        state.blueprint * (evaluate_plan(*state))
        }).sum();
    println!("result: {}", r);
}
