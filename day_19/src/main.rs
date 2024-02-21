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

            //macro_recult.unwrap();

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
        .map(|blueprint| {
            equality_level(&blueprint)
        })
        .sum();

    println!("Sum of blueprints' equality levels is: {}", result)
}

fn equality_level(blueprint: &Blueprint) -> i32 {
    blueprint.id * max_geodes_open(blueprint)
}

fn max_geodes_open(blueprint: &Blueprint) -> i32 {
    todo!()
}
