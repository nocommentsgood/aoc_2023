pub fn cube_condundrum(puzzle: &str) -> u32 {
    const RED_CUBES: u32 = 12;
    const GREEN_CUBES: u32 = 13;
    const BLUE_CUBES: u32 = 14;
    let mut sum = 0;

    let sanit = puzzle.replace("Game ", "");
    let sanit = sanit.replace("reen", "");
    let sanit = sanit.replace("ed", "");
    let sanit = sanit.replace("lue", "");
    let sanit = sanit.replace(" ", "");
    let sanit = sanit.replace(";", ",");

    let v: Vec<&str> = sanit.lines().collect();

    for i in 0..v.len() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let game_index = v[i].find(":");

        if game_index.is_some() {
            let (id, puz) = v[i].split_at(game_index.unwrap());
            let puz = puz.replace(":", "");

            'rep: for cube in puz.split(',') {
                let (num, color) = cube.split_at(cube.len() - 1);

                if let Ok(num) = num.parse::<u32>() {
                    match color {
                        "r" => red = num,
                        "g" => green = num,
                        "b" => blue = num,
                        _ => eprintln!("Bad color parsing: {}", color),
                    }
                }

                if !(red <= RED_CUBES && blue <= BLUE_CUBES && green <= GREEN_CUBES) {
                    break 'rep;
                }
            }

            if red <= RED_CUBES && blue <= BLUE_CUBES && green <= GREEN_CUBES {
                if let Ok(n) = id.parse::<u32>() {
                    sum += n;
                }
            }
        }
    }

    sum
}
