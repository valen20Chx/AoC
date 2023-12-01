// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

fn main() {
    let input = include_str!("./day1.txt");

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

    println!("{}", result);
}
