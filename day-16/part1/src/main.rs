use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<String>,
}

type ValveId = String;
type ValveMap = HashMap<ValveId, Valve>;
type PathLengths = HashMap<(ValveId, ValveId), u32>;

fn parse_input(text: &String) -> ValveMap {
    let mut result = ValveMap::new();

    let valve_def_str = r"Valve (.{2}) has flow rate=(\d+); tunnels? leads? to valves? (.{2}(?:, .{2})*)";
    let valve_def = Regex::new(valve_def_str).unwrap();
    let name_re = Regex::new(r"[[:upper:]]{2}").unwrap();

    for c in valve_def.captures_iter(text) {
        result.insert(
            c[1].to_string(),
            Valve{
                flow_rate: c[2].parse().unwrap(),
                tunnels: name_re.captures_iter(&c[3].to_string()).map(|c| c[0].to_string()).collect()
            }
        );

    }

    result
}

fn measure_path(map: &ValveMap, from: &ValveId, to: &ValveId) -> u32 {
    let mut path_length: u32 = 0;
    type ValveIdSet = HashSet::<ValveId>;
    let mut explored = ValveIdSet::new();
    let mut just_explored = ValveIdSet::new();

    just_explored.insert(from.clone());

    if from == to {
        return 0;
    }

    'map_search: loop {
        path_length += 1;
        let mut next_explored = ValveIdSet::new();
        for id in just_explored.iter() {
            let reachable: ValveIdSet = map.get(id).unwrap().tunnels.iter().cloned().collect();
            next_explored = next_explored.union(&reachable).cloned().collect();
        }
        for id in next_explored.iter() {
            if id == to {
                break 'map_search;
            }
        }
        just_explored = next_explored.difference(&explored).cloned().collect();
        explored = explored.union(&next_explored).cloned().collect();
    }
    
    path_length
}

fn measure_paths(map: &ValveMap) -> PathLengths {
    let mut result = PathLengths::new();

    for from in map.keys() {
        for to in map.keys() {
            result.insert((from.clone(), to.clone()), measure_path(map, &from, &to));
        }
    }

    result
}

struct Optimizer {
    map: ValveMap,
    paths: PathLengths,
}

impl Optimizer {
    fn flow_rate_of(&self, id: &String) -> u32 {
        self.map.get(id).unwrap().flow_rate
    }

    fn path_from_to(&self, from: &String, to: &String) -> u32 {
        *self.paths.get(&(from.clone(), to.clone())).unwrap()
    }

    fn gen_list_of_opened(&self) -> Vec<&String> {
        let mut opened: Vec<&String> = self.map.keys().filter(|k| self.flow_rate_of(k) > 0).collect();
        opened.sort_by(|l, r| self.flow_rate_of(r).partial_cmp(&self.flow_rate_of(l)).unwrap());
        opened
    }

    fn find_best_path_score(&self, pos: &String, time: u32, opened: &Vec<&String>, score: u32) -> u32 {
        let mut best_score: u32 = score;

        for (i, id) in opened.iter().enumerate() {
            // we are going to id, open it and evaluate options
            let time_to_open_id = self.path_from_to(pos, id) + 1;
            if time_to_open_id >= time {
                continue;
            }
            let id_time = time - time_to_open_id;
            let id_score = self.flow_rate_of(id) * id_time;
            let mut next_opened = opened.clone();
            next_opened.remove(i);

            let next_score = self.find_best_path_score(id, id_time, &next_opened, score + id_score);
            if next_score > best_score {
                best_score = next_score;
            }
        }

        best_score
    }
}


fn main() {
    let text = std::fs::read_to_string("../input.txt").expect("cannot load input file");
    let map = parse_input(&text);
    let paths = measure_paths(&map);

    let optimizer = Optimizer{map: map, paths: paths};
    let opened = optimizer.gen_list_of_opened();
    println!("best path score: {}", optimizer.find_best_path_score(&"AA".to_string(), 30, &opened, 0));
}
