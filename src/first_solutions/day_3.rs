// when a special charater is found, search in a 'box'
// pattern from the character
//
// for the first iteration, will operate under the assumption
// that each special character that is adjacent to a number,
// is adjacent to exactly that one number
// as well as each number having a length of 3

#![allow(unused)]
pub fn gear_ratio_solution(input: &str) -> u32 {
    let mut sum = 0;

    let line_vec: Vec<&str> = input.lines().collect();

    for i in 0..line_vec.len() {
        for (j, c) in line_vec[i].char_indices() {
            if !char::is_digit(c, 10) && c != '.' {
                check_above(&line_vec, j, i)
            }
        }
    }
    sum
}

fn check_above(input: &[&str], cur_index: usize, line_index: usize) {
    let mut sum = 0;
    let num = input[line_index - 1].get(cur_index - 1..cur_index + 2);
    if num.is_some() {
        for (i, c) in num.unwrap().char_indices() {
        }
    }

    let top_line = input[line_index - 1].get(cur_index - 3.. cur_index + 3);
    if top_line.is_some() {
        let top_vec = Vec::from(top_line.unwrap());
        // dbg!(top_vec.len());
    }

}
