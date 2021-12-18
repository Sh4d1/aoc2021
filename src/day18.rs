#[derive(Debug, Clone)]
pub enum Fish {
    N(usize),
    P(Box<Fish>, Box<Fish>),
}
use Fish::*;

impl Fish {
    fn insert(&mut self, v: usize, left: bool) {
        match self {
            N(n) => *n += v,
            P(l, _) if left => l.insert(v, left),
            P(_, r) => r.insert(v, left),
        }
    }

    fn reduce(mut self) -> Self {
        while self.try_explode(0).0 || self.try_split() {}
        self
    }

    fn try_split(&mut self) -> bool {
        match self {
            P(l, r) => l.try_split() || r.try_split(),
            N(n) if *n >= 10 => {
                *self = P(
                    box N((*n as f64 / 2.0).floor() as usize),
                    box N((*n as f64 / 2.0).ceil() as usize),
                );
                true
            }
            _ => false,
        }
    }

    fn try_explode(&mut self, depth: usize) -> (bool, Option<usize>, Option<usize>) {
        let mut zero = false;
        let res = match self {
            N(_) => (false, None, None),
            P(box N(l), box N(r)) if depth >= 4 => {
                zero = true;
                (true, Some(*l), Some(*r))
            }
            P(l, r) => {
                if let (true, rl, mut rr) = l.try_explode(depth + 1) {
                    if let Some(v) = rr {
                        r.insert(v, true);
                        rr = None;
                    }
                    (true, rl, rr)
                } else if let (true, mut rl, rr) = r.try_explode(depth + 1) {
                    if let Some(v) = rl {
                        l.insert(v, false);
                        rl = None;
                    }
                    (true, rl, rr)
                } else {
                    (false, None, None)
                }
            }
        };
        // maybe there is a better way /shrug
        if zero {
            *self = N(0);
        }
        res
    }

    fn magnitude(&self) -> usize {
        match self {
            P(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
            N(n) => *n,
        }
    }
}

fn parse_fish(input: &str) -> Fish {
    if let Ok(n) = input.parse::<usize>() {
        return N(n);
    }
    let (mut p, mut o) = (0, 0);
    input.chars().enumerate().for_each(|(i, c)| match c {
        '[' => o += 1,
        ']' => o -= 1,
        ',' if o == 1 => p = i,
        _ => (),
    });
    Fish::P(
        box parse_fish(&input[1..p]),
        box parse_fish(&input[p + 1..input.len() - 1]),
    )
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Fish> {
    input.lines().map(|l| parse_fish(l)).collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Fish]) -> usize {
    input[2..]
        .iter()
        .fold(P(box input[0].clone(), box input[1].clone()), |acc, e| {
            P(box acc.reduce(), box e.clone())
        })
        .reduce()
        .magnitude()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Fish]) -> usize {
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            max = max.max(
                P(box input[i].clone(), box input[j].clone())
                    .reduce()
                    .magnitude(),
            );
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            )),
            4140
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            )),
            3993
        )
    }
}
