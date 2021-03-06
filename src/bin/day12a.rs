use std::collections::VecDeque;
use std::collections::HashMap;

pub fn parse_rules(rules: Vec<String>) ->
        HashMap<(bool, bool, bool, bool, bool), bool> {
    let mut rules_map = HashMap::new();
    for rule in rules {
        let r = rule
            .replace(" ", "")
            .replace("=", "")
            .replace(">", "")
            .chars().map(|p| p == '#').collect::<Vec<bool>>();
        rules_map.insert((r[0], r[1], r[2], r[3], r[4]), r[5]);
    }
    rules_map
}

pub fn main() {
    let input = include_str!("../../input/day12a.txt");
    let input: Vec<String> = input
        .lines()
        .map(|l| l.to_string())
        .collect();
    let initial_state = &input[0]
        .split(' ')
        .map(|w| w.to_string())
        .collect::<Vec<String>>()[2];
    let mut state = initial_state.chars().map(|p| p == '#').collect::<VecDeque<bool>>();
    let rules = parse_rules(input[2..].to_vec());
    let generations = 20;
    let mut starting = 0isize;
    for _ in 0..generations {
        starting -= 2;
        for _ in 0..2 {
            state.push_front(false);
            state.push_back(false);
        }
        let previous = state.clone();
        let pots_num = state.len();
        for i in 0..pots_num {
            let pot_state = {
                if i == 0 {
                    (false, false, previous[i], previous[i+1], previous[i+2])
                } else if i == 1 {
                    (false, previous[i-1], previous[i], previous[i+1], previous[i+2])
                } else if i == pots_num - 2 {
                    (previous[i-2], previous[i-1], previous[i], previous[i+1], false)
                } else if i == pots_num - 1 {
                    (previous[i-2], previous[i-1], previous[i], false, false)
                } else {
                    (previous[i-2], previous[i-1], previous[i], previous[i+1], previous[i+2])
                }
            };
            state[i] = *rules.get(&pot_state).unwrap();
        }
    }
    let mut score = 0isize;
    for (i, pot) in state.iter().enumerate() {
        if *pot {
            score += i as isize + starting;
        }
    }
    println!("Sum of all pots is {}", score);
}
