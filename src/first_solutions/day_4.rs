pub fn scratchcard_solution(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (_, card) = line.split_at(10);
        let (left, right) = card.split_at(30);
        let winning_nums: Vec<u32> = left
            .split(' ')
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();
        let elf_nums: Vec<u32> = right
            .split(' ')
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();

        let nums: Vec<&u32> = elf_nums
            .iter()
            .filter(|n| winning_nums.contains(n))
            .collect();

        let points = match nums.len() {
            0 => 0,
            1 => 1,
            _ => {
                let mut points = 1;
                for _ in &nums[1..] {
                    points *= 2;
                } 
                points
            }
        };

        sum += points;
    }
    sum
}
