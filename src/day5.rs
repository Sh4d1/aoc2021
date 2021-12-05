use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::error::Error;
use std::collections::HashMap;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap(); 
}

pub struct Vent {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl FromStr for Vent {
    type Err = Box<dyn Error>; 

    fn from_str(s: &str) -> Result<Vent, Self::Err> {
        let cmd = REGEX.captures(s).unwrap();
        Ok(Vent{
            x1: cmd[1].parse::<i64>().unwrap(),
            y1: cmd[2].parse::<i64>().unwrap(),
            x2: cmd[3].parse::<i64>().unwrap(),
            y2: cmd[4].parse::<i64>().unwrap(),
        })
    }
}
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Vent> {
    input.lines().map(|l| l.parse::<Vent>().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Vent>) -> usize {
    let mut res = HashMap::new();
    input.iter()
        .filter(
            |v| 
            v.x1 == v.x2 || v.y1 == v.y2
        )
        .for_each(
            |v|
            {
                let mut x1 = v.x1;
                let mut x2 = v.x2;
                let mut y1 = v.y1;
                let mut y2 = v.y2;
                if (x1 == x2) {
                    if y2 < y1 {
                        (y1, y2) = (y2, y1); 
                    }
                    for y in y1..(y2+1) {
                        *res.entry((v.x1, y)).or_insert(0) += 1;
                    }
                }
                else if (y1 == y2) {
                    if x2 < x1 {
                        (x1, x2) = (x2, x1); 
                    }
                    for x in x1..(x2+1) {
                        *res.entry((x, v.y1)).or_insert(0) += 1;
                    }
                }
            }
        );
    res.iter().filter(|(_, v)| *v > &1).collect::<Vec<_>>().len()
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Vent>) -> usize {
    let mut res = HashMap::new();
    input.iter()
        .for_each(
            |v|
            {
                let mut x1 = v.x1;
                let mut x2 = v.x2;
                let mut y1 = v.y1;
                let mut y2 = v.y2;
                if (x1 == x2) {
                    if y2 < y1 {
                        (y1, y2) = (y2, y1); 
                    }

                    for y in y1..(y2+1) {
                        *res.entry((v.x1, y)).or_insert(0) += 1;
                    }
                }
                else if (y1 == y2) {
                    if x2 < x1 {
                        (x1, x2) = (x2, x1); 
                    }

                    for x in x1..(x2+1) {
                        *res.entry((x, v.y1)).or_insert(0) += 1;
                    }
                }
                else {
                    let dx = x2-x1;
                    let dy = y2-y1;
                    while x1 != x2 && y1!=y2 {
                        *res.entry((x1, y1)).or_insert(0) += 1;
                        if dx > 0 {
                            x1 += 1;
                        } else {
                            x1 -= 1;
                        }
                        if dy > 0 {
                            y1 += 1;
                        } else {
                            y1 -= 1;
                        }
                    }
                    *res.entry((x1, y1)).or_insert(0) += 1;
                }
            }
        );
    res.iter().filter(|(_, v)| *v > &1).collect::<Vec<_>>().len()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            )),
            5
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            )),
            12
        )
    }

}
