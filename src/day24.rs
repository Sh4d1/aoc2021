use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Insruction {
    Inp(Variable),
    Add(Variable, Variable),
    Mul(Variable, Variable),
    Div(Variable, Variable),
    Mod(Variable, Variable),
    Eql(Variable, Variable),
}
use Insruction::*;

impl Insruction {
    fn get_right(&self) -> isize {
        match self {
            Add(_, Value(x)) => *x,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
    Value(isize),
}
use Variable::*;

fn get_diff(instructions: &[Insruction]) -> Vec<(bool, isize, isize)> {
    instructions
        .chunks(18)
        .map(|c| {
            (
                c[4] == Div(Z, Value(1)),
                c[5].get_right(),
                c[15].get_right(),
            )
        })
        .collect()
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Insruction> {
    input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            let op = l.next().unwrap();
            let a = match l.next().unwrap().chars().next().unwrap() {
                'w' => W,
                'x' => X,
                'y' => Y,
                'z' => Z,
                _ => unreachable!(),
            };
            let b = match l.next().unwrap_or("0") {
                "w" => W,
                "x" => X,
                "y" => Y,
                "z" => Z,
                v => Value(v.parse::<isize>().unwrap()),
            };
            match op {
                "inp" => Inp(a),
                "add" => Add(a, b),
                "mul" => Mul(a, b),
                "div" => Div(a, b),
                "mod" => Mod(a, b),
                "eql" => Eql(a, b),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn find_model(input: &[Insruction]) -> (usize, usize) {
    let diffs = get_diff(input);
    let mut relations = HashMap::new();
    let mut base26 = vec![];

    for (i, d) in diffs.iter().enumerate() {
        if d.0 {
            base26.push((i, d.2));
        } else {
            let v = base26.pop().unwrap();
            relations.insert(i, (v.0, v.1 + d.1));
        }
    }

    let mut w_min = 0;
    let mut w_max = 0;
    for (k, v) in relations {
        let mut min = (10, 0);
        let mut max = (0, 0);
        for i in 1..10 {
            if i + v.1 <= 9 && i + v.1 >= 1 {
                if i + v.1 < min.0 {
                    min = (i + v.1, i);
                }
                if i + v.1 > max.0 {
                    max = (i + v.1, i);
                }
            }
        }
        w_max += 10usize.pow(13 - k as u32) * max.0 as usize;
        w_max += 10usize.pow(13 - v.0 as u32) * max.1 as usize;
        w_min += 10usize.pow(13 - k as u32) * min.0 as usize;
        w_min += 10usize.pow(13 - v.0 as u32) * min.1 as usize;
    }
    (w_max, w_min)
}

#[aoc(day24, part1)]
pub fn part1(input: &[Insruction]) -> usize {
    find_model(input).0
}

#[aoc(day24, part2)]
pub fn part2(input: &[Insruction]) -> usize {
    find_model(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator("")), 0)
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator("")), 0)
    }
}
