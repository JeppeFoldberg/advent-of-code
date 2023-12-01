fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(input));
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
