pub fn trebuchet(puzzle: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;
    let mut total = 0;

    for line in puzzle.lines() {
        for c in line.chars() {
            if c.is_ascii_digit() {
                first = char::to_digit(c, 10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last = char::to_digit(c, 10).unwrap();
                break;
            }
        }

        total += format!("{}{}", first, last).parse::<u32>().unwrap();
    }

    total
}
