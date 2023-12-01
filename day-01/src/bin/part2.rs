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
    let result = ac.replace_all(input, replace_with);
    
    // let _ = write("replaced_input.txt", result.clone());
    return format!("{}", part1(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1_works() {
	assert_eq!(true, true)
    }

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
