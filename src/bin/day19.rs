use std::{str::FromStr, fmt::Display};

use aoc2022::get_input;

const MINUTES_LIMIT: u32 = 24;

fn main() {
    let input = get_input().unwrap();

    let blueprints: Vec<_> = input.split('\n')
        .map(Blueprint::from_str)
        .map(Result::unwrap)
        .collect();

    for bp in blueprints {
        let start_mins = MINUTES_LIMIT - (bp.clay_rob_cost + 1);
        let mut stash = Stash::default();
        let mut production = Stash {
            ore: 1,
            clay: 1,
            ..Stash::default()
        };

        find_max_geodes(&bp, &mut stash, &mut production, start_mins);
    }

    println!("asdf");
}

fn find_max_geodes(bp: &Blueprint, stash: &mut Stash, prod: &mut Stash, min_left: u32) -> Geode {


    0
}

#[derive(Default)]
struct Stash {
    ore: Ore,
    clay: Clay,
    obs: Obsidian,
    geode: Geode,
}

struct Blueprint {
    id: usize,
    ore_rob_cost: Ore,
    clay_rob_cost: Ore,
    obs_rob_cost: (Ore, Clay),
    geode_rob_cost: (Ore, Obsidian),
}

type Ore = u32;
type Clay = u32;
type Obsidian = u32;
type Geode = u32;

macro_rules! parse_into {
    ($to_check:ident, $prefix:literal, $res:ident) => {
        let (tok, rest) = $to_check.split_at($prefix.len());
        debug_assert_eq!(tok, $prefix);

        let $res = rest.parse().map_err(|_| BlueprintParseError)?;
    };
    ($to_check:ident, $prefix:literal, $res:ident, $rest:literal) => {
        let (tok, rest) = $to_check.split_at($prefix.len());
        debug_assert_eq!(tok, $prefix);

        let (to_parse, rest) = rest.split_once(' ').ok_or(BlueprintParseError)?;
        let $res = to_parse.parse().map_err(|_| BlueprintParseError)?;
        debug_assert_eq!(rest, $rest);
    };
    ($to_check:ident, $prefix:literal, $res:ident, $infix:literal, $res2:ident, $rest2:literal) => {
        let (tok, rest) = $to_check.split_at($prefix.len());
        debug_assert_eq!(tok, $prefix);

        let (to_parse, rest) = rest.split_once(' ').ok_or(BlueprintParseError)?;
        let $res = to_parse.parse().map_err(|_| BlueprintParseError)?;

        let (rest, rest2) = rest.split_at($infix.len());
        debug_assert_eq!(rest, $infix);

        let (to_parse2, rest2) = rest2.split_once(' ').ok_or(BlueprintParseError)?;
        let $res2 = to_parse2.parse().map_err(|_| BlueprintParseError)?;
        debug_assert_eq!(rest2, $rest2);
    };
}

impl FromStr for Blueprint {
    type Err = BlueprintParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_str, contents) = s.split_once(':').ok_or(BlueprintParseError)?;
        parse_into!(id_str, "Blueprint ", id);

        let mut iter = contents.split('.');
        let rob_cost = iter.next().ok_or(BlueprintParseError)?;
        parse_into!(rob_cost, " Each ore robot costs ", ore_rob_cost, "ore");

        let rob_cost = iter.next().ok_or(BlueprintParseError)?;
        parse_into!(rob_cost, " Each clay robot costs ", clay_rob_cost, "ore");

        let rob_cost = iter.next().ok_or(BlueprintParseError)?;
        parse_into!(rob_cost, " Each obsidian robot costs ", obs_ore, "ore and ", obs_clay, "clay");

        let rob_cost = iter.next().ok_or(BlueprintParseError)?;
        parse_into!(rob_cost, " Each geode robot costs ", geode_ore, "ore and ", geode_obs, "obsidian");

        Ok(Self {
            id,
            ore_rob_cost,
            clay_rob_cost,
            obs_rob_cost: (obs_ore, obs_clay),
            geode_rob_cost: (geode_ore, geode_obs),
        })
    }
}

#[derive(Debug)]
struct BlueprintParseError;

impl Display for BlueprintParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error while parsing blueprint")
    }
}

impl std::error::Error for BlueprintParseError {}

enum Resource {
    Ore(usize),
    Clay(usize),
    Obsidian(usize),
}
