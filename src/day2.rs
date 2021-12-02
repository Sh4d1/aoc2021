use anyhow::{anyhow, Error};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

pub enum Cmd {
    Fwd(i64),
    Down(i64),
    Up(i64),
}

pub struct Submarine {
    depth: i64,
    h_pos: i64,
    aim: i64,
}

impl Submarine {
    pub fn zero() -> Self {
        Submarine{
            depth: 0,
            h_pos: 0,
            aim: 0,
        }
    }

    pub fn result(&self) -> i64 {
        self.depth*self.h_pos
    }
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"([a-z]+) (\d+)").unwrap();
}

impl FromStr for Cmd {
    type Err = Error; 

    fn from_str(s: &str) -> Result<Cmd, Self::Err> {
        let cmd = REGEX.captures(s).unwrap();
        match &cmd[1] {
            "forward" => Ok(Cmd::Fwd(cmd[2].parse::<i64>().unwrap())),
            "up" => Ok(Cmd::Up(cmd[2].parse::<i64>().unwrap())),
            "down" => Ok(Cmd::Down(cmd[2].parse::<i64>().unwrap())),
            _ => Err(anyhow!("Err")),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Cmd>  {
    input.lines().map(|l| l.parse::<Cmd>().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Cmd>) -> i64 {
    input.iter().fold(
        Submarine::zero(),
        |mut s, c| {
            match c {
                Cmd::Fwd(x) => s.h_pos += x,
                Cmd::Up(x) => s.depth -= x,
                Cmd::Down(x) => s.depth += x,
            };
            s
        }
    ).result()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Cmd>) -> i64 {
    input.iter().fold(
        Submarine::zero(),
        |mut s, c| {
            match c {
                Cmd::Fwd(x) => {
                    s.h_pos += x;
                    s.depth += s.aim * x;
                },
                Cmd::Up(x) => s.aim -= x,
                Cmd::Down(x) => s.aim += x,
            };
            s
        }
    ).result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )),
            150
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )),
            900
        )
    }

}
