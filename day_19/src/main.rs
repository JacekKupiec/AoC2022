use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use scanf::sscanf;

struct Blueprint {
    id: i32,
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost:i32,
    geode_robot_obsidian_cost: i32
}

impl Blueprint {
    fn new(
        id: i32,
        ore_robot_ore_cost: i32,
        clay_robot_ore_cost: i32,
        obsidian_robot_ore_cost: i32,
        obsidian_robot_clay_cost: i32,
        geode_robot_ore_cost:i32,
        geode_robot_obsidian_cost: i32) -> Self {
        Self {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost
        }
    }
}

#[derive(Hash)]
struct ResourcesInventory {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32
}

#[derive(Hash)]
struct ResourcesIncrements {
    ore_increment: i32,
    clay_increment: i32,
    obsidian_increment: i32,
    geode_increment: i32
}

fn main() {
    let file = File::open("small_input.txt").expect("Input file must exists");
    let buffer = BufReader::new(file);

    let result: i32 = buffer.lines()
        .map(|line_result| {
            let mut blueprint_id = 0;
            let mut ore_robot_cost  = 0;
            let mut clay_robot_cost = 0;
            let mut obsidian_robot_ore_cost = 0;
            let mut obsidian_robot_clay_cost = 0;
            let mut geode_robot_ore_cost = 0;
            let mut geode_robot_obsidian_cost = 0;

            let line = line_result.expect("Line has to be valid!");

            let macro_recult = sscanf!(
                line.trim_end(),
                "Blueprint {i32}: Each ore robot costs {i32} ore. Each clay robot costs {i32} ore. Each obsidian robot costs {i32} ore and {i32} clay. Each geode robot costs {i32} ore and {i32} obsidian.",
                blueprint_id,
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_ore_cost,
                obsidian_robot_clay_cost,
                geode_robot_ore_cost,
                geode_robot_obsidian_cost);

            macro_recult.unwrap();

            return Blueprint::new(
                blueprint_id,
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_ore_cost,
                obsidian_robot_clay_cost,
                geode_robot_ore_cost,
                geode_robot_obsidian_cost
            );
        })
        .map(|blueprint| equality_level(&blueprint))
        .sum();

    println!("Sum of blueprints' equality levels is: {}", result)
}

fn equality_level(blueprint: &Blueprint) -> i32 {
    let mut resources_inventory = ResourcesInventory {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0
    };

    let mut resources_increments = ResourcesIncrements {
        ore_increment: 1,
        clay_increment: 1,
        obsidian_increment: 1,
        geode_increment: 1
    };

    let mut cache = HashMap::new();

    blueprint.id * max_geodes_open(blueprint, &mut cache, &mut resources_inventory, &mut resources_increments, 24)
}

fn max_geodes_open(
    blueprint: &Blueprint,
    cache: &mut HashMap<(ResourcesInventory, ResourcesIncrements, i32), i32>,
    resources: &mut ResourcesInventory,
    resources_increments: &mut ResourcesIncrements, time: i32)
    -> i32 {
    if time <= 0 {
        return 0;
    }

    if let Some(result) = cache.contains_key(&(resources, resources_increments, time)) {
        return result;
    }
}
