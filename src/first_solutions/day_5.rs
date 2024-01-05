use std::collections::HashMap;

pub fn seed_fertilizer(input: &str) -> u32 {
    // parse first line of input to get seeds
    // can prob move this to another function
    let seed_line = input.lines().collect::<Vec<&str>>();
    let t: &[_] = &['s', 'e', 'd', ':', ' '];
    let seeds = seed_line[0]
        .trim_matches(t)
        .split_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    // parse input data to a map
    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .join("\n");
    let map_data = parse_input_to_map(input.as_str());
    let keys = parse_keys(input.as_str());
    let mut results: Vec<u32> = Vec::new();

    // surely there is a more concise way to do this
    for seed in seeds {
        let a = solve_given_seed(seed, map_data.get(keys[0]).unwrap());
        let a = solve_given_seed(a, map_data.get(keys[1]).unwrap());
        let a = solve_given_seed(a, map_data.get(keys[2]).unwrap());
        let a = solve_given_seed(a, map_data.get(keys[3]).unwrap());
        let a = solve_given_seed(a, map_data.get(keys[4]).unwrap());
        let a = solve_given_seed(a, map_data.get(keys[5]).unwrap());
        let a = solve_given_seed(a, map_data.get(keys[6]).unwrap());
        results.push(a);
    }
    return results.iter().min().unwrap().to_owned();
}

fn parse_keys(s: &str) -> Vec<&str> {
    let v: Vec<Vec<(u32, u32, u32)>> = Vec::new();
    let mut keys: Vec<&str> = Vec::new();
    let mut name = "";
    for line in s.lines() {
        if line.ends_with(':') {
            name = line.trim_end_matches(&[' ', 'm', 'a', 'p', ':']);
            keys.push(name);
        }
    }

    keys
}

fn parse_input_to_map(s: &str) -> HashMap<&str, Vec<(u32, u32, u32)>> {
    let mut map: HashMap<&str, Vec<(u32, u32, u32)>> = HashMap::new();
    let mut map_name = "";
    let mut keys: Vec<&str> = Vec::new();

    for (i, line) in s.lines().enumerate() {
        if line.ends_with(':') {
            map_name = line.trim_end_matches(&[' ', 'm', 'a', 'p', ':']);
            map.insert(map_name, Vec::new());
        } else {
            let nums: Vec<u32> = line
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();

            if map.get_mut(map_name).is_some() {
                map.get_mut(map_name)
                    .unwrap()
                    .push((nums[0], nums[1], nums[2]));
            }
        }
    }
    map
}

fn solve_given_seed(seed: u32, map: &[(u32, u32, u32)]) -> u32 {
    for tup in map.iter() {
        let max_source = tup.1 + (tup.2 - 1);
        if tup.1 <= seed && seed <= max_source {
            let offset = max_source - seed;
            let max_dest = tup.0 + (tup.2 - 1);
            let ans = max_dest - offset;
            return ans;
        }
    }
    seed
}
