use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::error::Error;
use num::signum;
use ndarray::Array2;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap(); 
}

pub struct Vent {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    current: (i64, i64),
    x_d: i64,
    y_d: i64,
}

impl Vent {
    pub fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        Vent{
            x1,
            y1,
            x2,
            y2,
            x_d: signum(x2-x1),
            y_d: signum(y2-y1),
            current: (-1, -1),
        }
    }
    pub fn is_diag(&self) -> bool {
        self.x_d != 0 && self.y_d != 0
    }
}

impl Iterator for &mut Vent {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == (-1, -1) {
            self.current = (self.x1, self.y1);
            return Some(self.current)
        }
        if self.current.0 == self.x2 && self.current.1 == self.y2 {
            return None;
        }
        self.current = (self.current.0 + self.x_d, self.current.1 + self.y_d);
        Some(self.current)
    }
}

impl FromStr for Vent {
    type Err = Box<dyn Error>; 

    fn from_str(s: &str) -> Result<Vent, Self::Err> {
        let cmd = REGEX.captures(s).unwrap();
        Ok(Vent::new(
            cmd[1].parse::<i64>().unwrap(),
            cmd[2].parse::<i64>().unwrap(),
            cmd[3].parse::<i64>().unwrap(),
            cmd[4].parse::<i64>().unwrap(),
        ))
    }
}

pub struct Diagram {
    vents: Vec<Vent>,
    grid: Array2<u64>,
}

impl Diagram {
    pub fn new_1(input: &str) -> Self {
        Diagram::from_vents(input.lines().map(|l| l.parse::<Vent>().unwrap()).filter(|v| !v.is_diag()).collect())
    }
    pub fn new(input: &str) -> Self {
        Diagram::from_vents(input.lines().map(|l| l.parse::<Vent>().unwrap()).collect())
    }
    pub fn from_vents(vents: Vec<Vent>) -> Self {
        let (max_x, max_y) = vents.iter().fold((0,0), |mut acc, v| {
            let lmx = std::cmp::max(v.x1, v.x2) as usize;
            let lmy = std::cmp::max(v.y1, v.y2) as usize;
            if acc.0 < lmx {
                acc.0 = lmx;
            }
            if acc.1 < lmy {
                acc.1 = lmy;
            }
            acc
        });
        Diagram{
            grid: Array2::from_elem((max_x+1, max_y+1), 0),
            vents,
        }
    }

    pub fn draw(&mut self) {
        for v in self.vents.iter_mut() {
            for (x,y) in v {
                *self.grid.get_mut((x as usize,y as usize)).unwrap() += 1;
            }
        }
    }

    pub fn count(&self) -> usize {
        self.grid.iter().fold(0, |acc, &n| {
            if n > 1 {
                acc + 1
            } else {
                acc
            }
        })
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut diag = Diagram::new_1(input);
    diag.draw();
    diag.count()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut diag = Diagram::new(input);
    diag.draw();
    diag.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(
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
            ),
            5
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(
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
            ),
            12
        )
    }

}
