use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../inputs/example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("../inputs/input.txt");

// this is going to be hell, I know;
pub fn solve() {
    // let input: Vec<Blueprint> = EXAMPLE.lines().map(Blueprint::parse).collect();
    let input: Vec<Blueprint> = INPUT.lines().map(Blueprint::parse).collect();
    let mut sum = 0;

    for b in input {
        let id = b.id;
        let max = max_geodes(b);

        sum += id*max;
    }

    println!("Day 19 part 1: {}", sum);
}

fn max_geodes(blueprint: Blueprint) -> u16 {
    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint.bots.iter().map(|cost| cost[i]).max().unwrap();
    }

    let max_time = 24;
    let mut max_geodes = 0;

    let mut q = VecDeque::new();
    q.push_back(State {
        materials: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(State {
        materials,
        bots,
        elapsed,
    }) = q.pop_front()
    {
        // for every bot cost, run simulation
        for i in 0..blueprint.bots.len() {
            if bots[i] == max_robots[i] {
                continue;
            }

            let costs = &blueprint.bots[i];
            // Find the limiting resource type for the costs.
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        // state has enough of current resource in inventory to cover that part of the target bot cost. 0 wait time
                        cost if cost <= materials[idx] => 0, // can afford to build robot
                        // no target bot type made yet
                        // we can't build it (it takes more than max_time to build it).
                        _ if bots[idx] == 0 => max_time + 1,
                        _ => (costs[idx] - materials[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            // if that choice would cause the time limit be to exceeded, skip
            // the + 1 is so the built bot has the chance to do something, it merely being built is not enough
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            // gather ores with previously available bots
            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = materials[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            // increase bot type for the bot we just built
            let mut new_bots = bots;
            new_bots[i] += 1;

            q.push_back(State {
                materials: new_inventory,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }

        let geodes = materials[3] + bots[3] * (max_time - elapsed);
        max_geodes = geodes.max(max_geodes);
    }

    max_geodes
}

#[derive(Debug, Clone)]
struct State {
    materials: [u16; 4],
    bots: [u16; 4],
    elapsed: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            x => panic!("Wrong string input {}", x),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u16,
    bots: [[u16; 4];4]
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        let (name, content) = line.split_once(":").unwrap();

        let id = name.split_whitespace().last().unwrap().parse().unwrap();

        let content: Vec<&str> = content.split(".").collect();
        let ore = Self::parse_cost(content[0]);
        let clay = Self::parse_cost(content[1]);
        let obsidian = Self::parse_cost(content[2]);
        let geode = Self::parse_cost(content[3]);

        Blueprint {
            id,
            bots: [ore, clay, obsidian, geode]
        }
    }

    fn parse_cost(s: &str) -> [u16; 4] {
        let content: Vec<&str> = s.split_ascii_whitespace().collect();
        let mut cost = [0;4];
        match Resource::parse(content[1]) {
            Resource::Ore | Resource::Clay => {
                let c = content[4].parse().unwrap();
                cost[0] = c;
            }
            Resource::Obsidian => {
                let ore_cost = content[4].parse().unwrap();
                let clay_cost = content[7].parse().unwrap();
                cost[0] = ore_cost;
                cost[1] = clay_cost;
            }
            Resource::Geode => {
                let ore_cost = content[4].parse().unwrap();
                let obsidian_cost = content[7].parse().unwrap();
                cost[0] = ore_cost;
                cost[2] = obsidian_cost;
            }
        }

        cost
    }

}

    
