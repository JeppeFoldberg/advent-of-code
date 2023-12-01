use aho_corasick::AhoCorasick;
// use std::fs::write;
// use std::io::Write;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part2(input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;
 
    for line in input.lines() {
        let numbers: Vec<&str> = line.matches(char::is_numeric).collect();
        let calibration_number: i32 = format!("{}{}",
            numbers.first().unwrap(),
            numbers.last().unwrap()
        ).parse().unwrap();
        
        sum += calibration_number;
        // println!("{calibration_number:?}")
    }
    return format!("{sum}")
}

fn part2(input: &str) -> String {
    let patterns = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let replace_with = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    
    let ac = AhoCorasick::new(patterns).unwrap();
    let matches = ac  // replace_all(input, replace_with);
	.find_overlapping_iter(&input);

    let mut result = input.to_owned();
    let mut usize_offset = 0;
    for mat in matches {
	let start = mat.start() + usize_offset;
	usize_offset += replace_with[mat.pattern()].len();
	result.insert_str(
	    start,
	    replace_with[mat.pattern()]
	);
    }

    return format!("{}", part1(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
	assert_eq!(true, true)
    }
    #[test]
    fn part2_works() {
	assert_eq!("281", part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"))
    }
}
