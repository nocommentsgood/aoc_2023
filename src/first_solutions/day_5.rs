#![allow(unused)]

use std::collections::HashMap;

pub fn seed_fertilizer(input: &str) {
    // parse first line of input to get seeds
    // can prob move this to another function
    let seed_line = input.lines().collect::<Vec<&str>>();
    let t: &[_] = &['s', 'e', 'd', ':', ' '];
    let seeds = seed_line[0].trim_matches(t).split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect::<Vec<u32>>();

    // parse input data to a map
    let input = input.lines().filter(|l| !l.is_empty()).collect::<Vec<&str>>().join("\n");
    let data = parse_input_to_map(input.as_str());

    let first = data.get("seed-to-soil").unwrap().first().unwrap();
}

fn parse_input_to_map(s: &str) -> HashMap<&str, Vec<(u32, u32, u32)>> {
    let mut map: HashMap<&str, Vec<(u32, u32, u32)>> = HashMap::new();
    let mut map_name = "";

    for line in s.lines() {
        if line.ends_with(':') {
            map_name = line.trim_end_matches(&[' ', 'm', 'a', 'p', ':']);
            map.insert(map_name, Vec::new());
        } else {
            let nums: Vec<u32> = line.split_whitespace().filter_map(|n| n.parse::<u32>().ok()).collect();

            if map.get_mut(map_name).is_some() {
            map.get_mut(map_name).unwrap().push((nums[0], nums[1], nums[2]));
            }
        }
    }
    map
}
