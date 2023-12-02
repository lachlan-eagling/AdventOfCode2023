fn main() {
    const INPUT: &str = include_str!("input");
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

fn part_1(input: &str) -> String {
    let mut results: Vec<u32> = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (mut p1, mut p2) = (0, line.len().saturating_sub(1));
        let mut val_a: Option<char> = None;
        let mut val_b: Option<char> = None;

        while val_a.is_none() || val_b.is_none() {
            if val_a.is_none() {
                val_a = line.chars().nth(p1).filter(|c| c.is_numeric());
                p1 = p1.saturating_add(1);
            }
            if val_b.is_none() {
                val_b = line.chars().nth(p2).filter(|c| c.is_numeric());
                p2 = p2.saturating_sub(1);
            }
        }

        if let (Some(a), Some(b)) = (val_a, val_b) {
            if let Ok(num) = format!("{}{}", a, b).parse::<u32>() {
                results.push(num)
            }
        }
    }

    let sum: u32 = results.iter().sum();
    return format!("{}", sum);
}

fn part_2(input: &str) -> String {
    const MATCH_TERMS: [&str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut results: Vec<u32> = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let (mut p1, mut p2) = (0, line.len().saturating_sub(1));
        let mut val_a: Option<&str> = None;
        let mut val_b: Option<&str> = None;

        while val_a.is_none() || val_b.is_none() {
            let mut temp_a: Option<&str> = None;
            if val_a.is_none() {
                temp_a = line.get(..p1);
                p1 = p1.saturating_add(1);
            }

            let mut temp_b: Option<&str> = None;
            if val_b.is_none() {
                temp_b = line.get(p2..line.len());
                p2 = p2.saturating_sub(1);
            }

            for s in MATCH_TERMS {
                if val_a.is_none() {
                    if let Some(x) = temp_a {
                        if x.contains(s) {
                            val_a = match_substring(s)
                        }
                    }
                }

                if val_b.is_none() {
                    if let Some(x) = temp_b {
                        if x.contains(s) {
                            val_b = match_substring(s)
                        }
                    }
                }
            }
        }

        if let (Some(a), Some(b)) = (val_a, val_b) {
            if let Ok(num) = format!("{}{}", a, b).parse::<u32>() {
                results.push(num)
            }
        }
    }

    let sum: u32 = results.iter().sum();
    return format!("{}", sum);
}

fn match_substring(s: &str) -> Option<&str> {
    match s {
        "1" | "one" => return Some("1"),
        "2" | "two" => return Some("2"),
        "3" | "three" => return Some("3"),
        "4" | "four" => return Some("4"),
        "5" | "five" => return Some("5"),
        "6" | "six" => return Some("6"),
        "7" | "seven" => return Some("7"),
        "8" | "eight" => return Some("8"),
        "9" | "nine" => return Some("9"),
        _ => return None,
    }
}
