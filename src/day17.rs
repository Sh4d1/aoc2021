use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
}

#[derive(Debug, Clone)]
pub struct Area {
    x: (isize, isize),
    y: (isize, isize),
    p: (isize, isize),
    v: (isize, isize),
}

impl Area {
    fn step(&mut self) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        match self.v.0 {
            x if x > 0 => self.v.0 -= 1,
            x if x < 0 => self.v.0 += 1,
            _ => (),
        }
        self.v.1 -= 1;
    }

    fn try_v(&mut self, vx: isize, vy: isize) -> (bool, isize) {
        let mut max_y = 0;
        self.v = (vx, vy);
        loop {
            self.step();
            if self.p.1 > max_y {
                max_y = self.p.1
            }
            if self.p.0 >= self.x.0
                && self.p.0 <= self.x.1
                && self.p.1 >= self.y.0
                && self.p.1 <= self.y.1
            {
                return (true, max_y);
            }
            if self.p.0 > self.x.1 || self.p.1 < self.y.0 {
                return (false, max_y);
            }
        }
    }

    fn try_all(&self) -> (isize, usize) {
        let mut max_y = 0;
        let mut n = 0;
        for x in 0..=self.x.1 {
            for y in self.y.0.min(self.y.1)..self.y.0.abs().max(self.y.1.abs()) {
                let (res, max) = self.clone().try_v(x, y);
                if res {
                    n += 1;
                    if max > max_y {
                        max_y = max;
                    }
                }
            }
        }
        (max_y, n)
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Area {
    let i = REGEX.captures(input.lines().next().unwrap()).unwrap();
    Area {
        v: (0, 0),
        p: (0, 0),
        x: (
            i[1].parse::<isize>().unwrap(),
            i[2].parse::<isize>().unwrap(),
        ),
        y: (
            i[3].parse::<isize>().unwrap(),
            i[4].parse::<isize>().unwrap(),
        ),
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &Area) -> isize {
    input.try_all().0
}

#[aoc(day17, part2)]
pub fn part2(input: &Area) -> usize {
    input.try_all().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator("target area: x=20..30, y=-10..-5")),
            45
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator("target area: x=20..30, y=-10..-5")),
            112
        )
    }
}
