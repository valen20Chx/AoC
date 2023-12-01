// Part 1 :
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

fn part_one(input: &str) -> u32 {
    let lines = input.lines();

    let result = lines
        .map(|line| -> u32 {
            let first_idx = line.find(char::is_numeric);
            let last_idx = line.rfind(char::is_numeric);

            match (first_idx, last_idx) {
                (Some(first), Some(last)) => {
                    let first = line.chars().nth(first).unwrap().to_digit(10).unwrap();
                    let last = line.chars().nth(last).unwrap().to_digit(10).unwrap();

                    first * 10 + last
                }
                _ => 0,
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();

    result
}

// Part 2 :
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
//
// What is the sum of all of the calibration values?

fn find_first_match<'a>(haystack: &'a str, needles: &'a [&'a str]) -> Option<(usize, &'a &'a str)> {
    for i in 0..haystack.len() {
        for substring in needles.iter().enumerate() {
            if haystack[i..].starts_with(substring.1) {
                return Some(substring);
            }
        }
    }
    None
}

fn find_last_match<'a>(haystack: &'a str, needles: &'a [&'a str]) -> Option<(usize, &'a &'a str)> {
    for i in 0..haystack.len() {
        for substring in needles.iter().enumerate() {
            if haystack[..haystack.len() - i].ends_with(substring.1) {
                return Some(substring);
            }
        }
    }
    None
}

fn get_num_strs<'a>() -> Vec<&'a str> {
    let num_strs = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    return num_strs;
}

fn part_two(input: &str) -> u32 {
    let num_strs = get_num_strs();

    let lines = input.lines();

    let result = lines
        .map(|line| {
            let first_idx = find_first_match(line, &num_strs);
            let last_idx = find_last_match(line, &num_strs);

            (first_idx, last_idx)
        })
        .map(|(first_idx, last_idx)| -> u32 {
            match (first_idx, last_idx) {
                (Some(first), Some(last)) => {
                    let first: u32 = (first.0 as u32) % 10;
                    let last: u32 = (last.0 as u32) % 10;

                    first * 10 + last
                }
                _ => 0,
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();

    result
}

fn main() {
    let input = include_str!("./day1.txt");
    println!("{}", part_one(input));

    println!("{}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(part_one(input), 142);
    }

    #[test]
    fn test_part_two() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part_two(input), 281);
    }

    #[test]
    fn test_find_last_match() {
        let num_strs = get_num_strs();
        assert_eq!(
            find_last_match("7pqrstsixteen", &num_strs),
            Some((6, &"six"))
        );
    }
}
