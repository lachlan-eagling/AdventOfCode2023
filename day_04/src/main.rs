use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input");
    println!("☃️ Part 1: {}", part_1(INPUT));
    println!("🎄 Part 2: {}", part_2(INPUT));
}

fn process_card(line: &str) -> Vec<u32> {
    let card_parts: Vec<&str> = line[line.find(":").unwrap() + 1..line.len()]
        .split("|")
        .collect();

    let winning_set = HashSet::from_iter(
        card_parts[0]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap()),
    );

    let card_set: HashSet<u32> = HashSet::from_iter(
        card_parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap()),
    );

    let result: Vec<u32> = winning_set.intersection(&card_set).cloned().collect();
    return result;
}

fn part_1(input: &str) -> u32 {
    let mut points = 0;
    for line in input.lines() {
        let won_values = process_card(line);
        let mut card_value = 0;
        for i in 0..won_values.len() {
            if i == 0 {
                // first one, initialize card value with 1.
                card_value = 1;
                continue;
            }
            card_value *= 2;
        }
        points += card_value;
    }
    return points;
}

fn part_2(input: &str) -> u32 {
    // Initialize a vector to track counts of cards. The index coresponds to the card number, with the value the count of copies.
    let mut card_counts: Vec<u32> = vec![1; 209]; // todo: set count to length of cards.

    // Loop through cards 1 by 1.
    for (i, card) in input.lines().enumerate() {
        // Now go through and process each copy of the card.
        let winners = process_card(card).len();
        for _ in 0..card_counts[i] {
            // Increment counts for following "won" card copies.
            for k in 0..winners {
                let next_idx = i + k + 1;
                card_counts[next_idx] = card_counts[next_idx] + 1;
            }
        }
    }

    // Summ and return total qty.
    return card_counts.iter().sum(); // card_counts.to_vec().iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: &str = include_str!("input");
        assert_eq!(part_1(input), 21158);
    }

    #[test]
    fn test_part_2() {
        let input: &str = include_str!("input");
        assert_eq!(part_2(input), 6050769);
    }
}
