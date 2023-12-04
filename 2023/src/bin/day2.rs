// Part 1 :
// Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
// 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

struct SubSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl SubSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn new_from_str(s: &str) -> Self {
        let mut set = Self::new(0, 0, 0);
        s.split(',').for_each(|l| {
            let info = l.trim().split(" ").collect::<Vec<&str>>();
            let count = info[0].parse::<u32>().ok();
            let color = info[1];

            match (count, color) {
                (Some(count), "red") => set.red += count,
                (Some(count), "green") => set.green += count,
                (Some(count), "blue") => set.blue += count,
                _ => (),
            }
        });
        set
    }
}

impl std::fmt::Display for SubSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}; g: {}; b: {}", self.red, self.green, self.blue)
    }
}

fn part_one(input: &str) -> u32 {
    let max_set = SubSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let lines = input.lines();

    lines
        .map(|line| -> bool {
            line.split_at(match line.find(':') {
                Some(i) => i + 1,
                None => return false,
            })
            .1
            .split(";")
            .map(SubSet::new_from_str)
            .all(|s| s.red <= max_set.red && s.green <= max_set.green && s.blue <= max_set.blue)
        })
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

// Part 2 :
// For each game, find the minimum set of cubes that must have been present.
// What is the sum of the power of these sets?
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

fn part_two(input: &str) -> u32 {
    let lines = input.lines();

    lines
        .map(|line| -> u32 {
            let max_set = line
                .split_at(match line.find(':') {
                    Some(i) => i + 1,
                    None => 0,
                })
                .1
                .split(";")
                .map(SubSet::new_from_str)
                .fold(
                    SubSet {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |mut acc, s| {
                        if s.red > acc.red {
                            acc.red = s.red;
                        }
                        if s.green > acc.green {
                            acc.green = s.green;
                        }
                        if s.blue > acc.blue {
                            acc.blue = s.blue;
                        }
                        acc
                    },
                );

            max_set.red * max_set.green * max_set.blue
        })
        .sum()
}

fn main() {
    let input = include_str!("./day2.txt");
    println!("{}", part_one(input));

    println!("{}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_one(input), 8);
    }

    #[test]
    fn test_part_two() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_two(input), 2286);
    }
}
