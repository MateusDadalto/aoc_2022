use std::collections::{BTreeSet, HashMap, VecDeque};

const EXAMPLE: &str = include_str!("../inputs/example.txt");
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let input = INPUT;
    let size = input.lines().count();
    let mut valves = HashMap::with_capacity(size);

    input.lines().for_each(|l| {
        let v = Valve::parse(l);
        valves.insert(v.id.clone(), v);
    });

    let distances = calculate_distances(&valves);
    let limit = 30;

    let mut possible_states = VecDeque::new();
    possible_states.push_back(State {
        opened: BTreeSet::new(),
        current: "AA",
        elapsed: 0,
        relieved: 0,
    });

    let mut max_relieve = 0;

    while let Some(State {
        opened,
        current,
        elapsed,
        relieved,
    }) = possible_states.pop_front()
    {
        let possible_valves = valves
            .values()
            .filter(|&v| v.has_flow && !opened.contains(v.id.as_str()));

        for next_valve in possible_valves {
            let distance = distances[current][next_valve.id.as_str()];
            let new_elapsed = elapsed + distance + 1;

            if new_elapsed >= limit {
                continue;
            }

            let new_relieved = relieved + ((limit - new_elapsed) * next_valve.flow_rate);
            max_relieve = max_relieve.max(new_relieved);

            let mut new_opened = opened.clone();
            new_opened.insert(next_valve.id.as_str());

            possible_states.push_back(State {
                opened: new_opened,
                current: &next_valve.id,
                elapsed: new_elapsed,
                relieved: new_relieved,
            });
        }
    }

    println!("Day 16 part 1: {max_relieve}");
}

// use Floyd-Warshall algorithm applied to hashmaps instead of arrays (keys instead of indexes)
fn calculate_distances(valves: &ValveMap) -> DistanceMap {
    let mut distances: DistanceMap = HashMap::with_capacity(valves.len());

    for k in valves.keys() {
        let columns: HashMap<String, u32> = valves.keys().map(|k| (k.clone(), u32::MAX)).collect();
        distances.insert(k.clone(), columns);
    }

    // fill base distances
    for (k, v) in valves {
        let vertex = distances.get_mut(k).unwrap().get_mut(k).unwrap();

        *vertex = 0;

        for i in v.links.iter() {
            let edge = distances.get_mut(k).unwrap().get_mut(i).unwrap();

            *edge = 1;
        }
    }

    for k in valves.keys() {
        for i in valves.keys() {
            for j in valves.keys() {
                let dist_sum = distances[i][k].saturating_add(distances[k][j]);
                if distances[i][j] > dist_sum {
                    let dist_i_j = distances.get_mut(i).unwrap().get_mut(j).unwrap();

                    *dist_i_j = dist_sum
                }
            }
        }
    }

    distances
}

type DistanceMap = HashMap<String, HashMap<String, u32>>;
type ValveMap = HashMap<String, Valve>;

struct State<'a> {
    opened: BTreeSet<&'a str>,
    current: &'a str,
    elapsed: u32,
    relieved: u32,
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: u32,
    has_flow: bool,
    links: Vec<String>,
}

impl Valve {
    fn parse(line: &str) -> Self {
        let (valve_info, link_info) = line.split_once(";").unwrap();
        let id = valve_info[6..8].to_string();
        let flow_rate = valve_info[23..].parse().unwrap();
        let links = link_info
            .split_whitespace()
            .filter(|s| s.chars().any(|c| matches!(c, 'A'..='Z')))
            .map(|s| s.replace(",", "").trim().to_string())
            .collect();

        Valve {
            id,
            flow_rate,
            has_flow: flow_rate != 0,
            links,
        }
    }
}
