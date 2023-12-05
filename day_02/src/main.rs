use std::cmp;

fn main() {
    const INPUT: &str = include_str!("input");
    println!("☃️ Part 1: {}", part_1(INPUT));
    println!("🎄 Part 2: {}", part_2(INPUT));
}

fn part_1(input: &str) -> u32 {
    let mut games_possible: Vec<u32> = Vec::new();

    for (i, line) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        let game_id: u32 = (i + 1) as u32;
        let mut game_possible = true;
        for subset in parse_game(line) {
            let cubes = subset.split(",");

            for mut cube in cubes {
                cube = cube.trim();
                let colour_qty: Vec<&str> = cube.split(" ").collect();
                let qty = colour_qty[0].parse::<u32>().unwrap();

                match colour_qty[1] {
                    "red" => {
                        if qty > 12 {
                            game_possible = false
                        }
                    }
                    "green" => {
                        if qty > 13 {
                            game_possible = false
                        }
                    }
                    "blue" => {
                        if qty > 14 {
                            game_possible = false
                        }
                    }
                    _ => {}
                }
            }
        }
        if game_possible {
            games_possible.push(game_id);
        }
    }

    return games_possible.iter().sum();
}

fn part_2(input: &str) -> u32 {
    let mut summed_powers: Vec<u32> = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut fewest_red = 0;
        let mut fewest_green = 0;
        let mut fewest_blue = 0;

        for subset in parse_game(line) {
            let cubes = subset.split(",");

            for mut cube in cubes {
                cube = cube.trim();
                let colour_qty: Vec<&str> = cube.split(" ").collect();
                let qty = colour_qty[0].parse::<u32>().unwrap();

                match colour_qty[1] {
                    "red" => {
                        fewest_red = cmp::max(fewest_red, qty);
                    }
                    "green" => {
                        fewest_green = cmp::max(fewest_green, qty);
                    }
                    "blue" => fewest_blue = cmp::max(fewest_blue, qty),
                    _ => {}
                }
            }
        }

        summed_powers.push(fewest_red * fewest_green * fewest_blue);
    }

    return summed_powers.iter().sum();
}

fn parse_game(line: &str) -> Vec<&str> {
    let cubes: Vec<&str> = line.split(":").collect();
    let cube_subsets = cubes[1].split(";");
    return cube_subsets.collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");

    #[test]
    fn test_part_1() {
        let input: &str = INPUT;
        assert_eq!(part_1(input), 2795);
    }

    #[test]
    fn test_part_2() {
        let input: &str = INPUT;
        assert_eq!(part_2(input), 75561);
    }
}
