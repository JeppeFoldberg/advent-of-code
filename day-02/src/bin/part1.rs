use nom::{
    IResult,
    branch::alt,
//    multi::many0,
    bytes::complete::tag,
    character::complete::digit1,
    sequence::tuple,
};

#[derive(Debug, Default)]
pub struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

// this is where we add the rules for a current game! 
impl Round {
  pub fn new(red: u8, green: u8, blue: u8) -> Option<Self> {
    if red <= 12 && green <= 13 && blue <= 14 {  // these are the rules! 
      Some(Round { red, green, blue })
    } else {
      None
    }
  }
}

fn parse_game(input: &str) -> IResult<&str, &str> {
    tag("Game ")(input)
}

fn parse_color(input: &str) -> IResult<&str, &str> {
    alt((
        tag("green"),
        tag("red"),
        tag("blue"),
    ))(input)
}

fn parse_round(input: &str) -> IResult<&str, &str> {
    tuple(
        parse_game,
        digit1,
        parse_color,
    )(input)
}


fn part1(input: &str) -> String {
    let test = Round {
        red: 1,
        blue: 2,
        ..Default::default()
    };
    dbg!(test);
    let one_line = input.lines().next().unwrap();
    let result = parse_round(one_line);
    dbg!(result);
    "todo()".to_string()
}

fn main() {
   let input = include_str!("./input.txt");
   let output = part1(input);
   dbg!(output);
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "88888888".to_string());
    }

    #[test]
    fn process_round_works() {
        assert_eq!(
           process_round("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
           true
        );
        assert_eq!(
           process_round("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
           false
        );
    }
}
