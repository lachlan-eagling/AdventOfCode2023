extern crate regex;

use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input");
    println!("🔔 Part 1: {}", part_1(INPUT));
    println!("🎁 Part 2: {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let sym_re = Regex::new(r"[^a-zA-Z0-9.]").unwrap();
    let num_re = Regex::new(r"\d+").unwrap();
    let mut nums: Vec<u32> = Vec::new();

    // 1. Parse lines by searching for numbers or symbols (regexp?), store the number along with the index of it in the line.
    let lines: Vec<&str> = input.lines().collect();
    let line_size: usize;
    match lines.first() {
        Some(inner) => line_size = inner.len(),
        None => panic!("Unexpected input data."),
    }

    for i in 0..lines.len() {
        let prev_line = lines[i.saturating_sub(1)];
        let curr_line = lines[i];
        let next_line_idx = if i.saturating_add(1) >= lines.len() {
            lines.len() - 1
        } else {
            i.saturating_add(1)
        };
        let next_line = lines[next_line_idx];

        for m in num_re.find_iter(curr_line) {
            let matched_num = m.as_str().parse::<u32>().unwrap();
            let start = m.start().saturating_sub(1);
            let end = if m.start().saturating_add(m.len() + 1) >= line_size {
                line_size
            } else {
                m.start().saturating_add(m.len() + 1)
            };

            let current_line_window = &curr_line[start..end];

            if sym_re.is_match(current_line_window) {
                nums.push(matched_num);
            }

            if prev_line != "" {
                let preceding_line_window = &prev_line[start..end];
                if sym_re.is_match(preceding_line_window) {
                    nums.push(matched_num);
                }
            }

            if next_line != "" {
                let following_line_window = &next_line[start..end];
                if sym_re.is_match(following_line_window) {
                    nums.push(matched_num);
                }
            }
        }
    }

    return nums.iter().sum();
}

fn part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let re_star = Regex::new(r"\*").unwrap();
    let re_number = Regex::new(r"\d+").unwrap();

    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        for mat in re_star.find_iter(line) {
            let match_pos = mat.start();

            let mut neighbouring_numbers: Vec<u32> = Vec::new();

            // 1. Get number matches in previous line and check if they overlap with match_pos-1 to match_pos+1
            let prev_line = lines[i.saturating_sub(1)];
            if i > 0 {
                // guard against double checking first line.
                for prev_line_match in re_number.find_iter(prev_line) {
                    let start = prev_line_match.start();
                    let end = prev_line_match.end() - 1;
                    if (start >= match_pos.saturating_sub(1)
                        && start <= match_pos.saturating_add(1))
                        || (end >= match_pos.saturating_sub(1)
                            && end <= match_pos.saturating_add(1))
                    {
                        //number overlaps the matched *, add to collection

                        neighbouring_numbers.push(prev_line_match.as_str().parse::<u32>().unwrap());
                    }
                }
            }

            // 2. Get number matches in current line and check if they overlap with match_pos-1 to match_pos+1
            for current_line_match in re_number.find_iter(line) {
                let start = current_line_match.start();
                let end = current_line_match.end() - 1;
                if (start >= match_pos.saturating_sub(1) && start <= match_pos.saturating_add(1))
                    || (end >= match_pos.saturating_sub(1) && end <= match_pos.saturating_add(1))
                {
                    neighbouring_numbers.push(current_line_match.as_str().parse::<u32>().unwrap());
                }
            }

            // 3. Get number matches in next line and check if they overlap with match_pos-1 to match_pos+1
            let next_line = lines[i.saturating_add(1)];
            for next_line_match in re_number.find_iter(next_line) {
                let start = next_line_match.start();
                let end = next_line_match.end() - 1;
                if (start >= match_pos.saturating_sub(1) && start <= match_pos.saturating_add(1))
                    || (end >= match_pos.saturating_sub(1) && end <= match_pos.saturating_add(1))
                {
                    neighbouring_numbers.push(next_line_match.as_str().parse::<u32>().unwrap());
                }
            }

            if neighbouring_numbers.len() == 2 {
                let a = neighbouring_numbers.first().unwrap();
                let b = neighbouring_numbers.last().unwrap();
                let ratio = a * b;
                total += ratio;
            }
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: &str = include_str!("input");
        assert_eq!(part_1(input), 540025);
    }

    #[test]
    fn test_part_2() {
        let input: &str = include_str!("input");
        assert_eq!(part_2(input), 84584891);
    }
}
