#[derive(PartialEq, Eq, Clone)]
pub enum ChunkType {
    P,
    B,
    C,
    A,
}
use ChunkType::*;

#[derive(Clone)]
pub struct Chunk(ChunkType, bool, usize, usize);

pub struct Line(Vec<Chunk>);

impl Line {
    fn get_corrupted_chunk(&self) -> Option<Chunk> {
        let mut visited = vec![];
        for c in self.0.iter() {
            if c.1 {
                visited.push(c);
                continue;
            }
            if visited.pop().unwrap().0 != c.0 {
                return Some((*c).clone());
            }
        }
        None
    }
    fn get_end(&self) -> Option<Vec<Chunk>> {
        if self.get_corrupted_chunk().is_some() {
            return None;
        }
        Some(
            self.0
                .iter()
                .fold(vec![], |mut acc, c| {
                    if c.1 {
                        acc.push(c.clone());
                    } else {
                        acc.pop();
                    }
                    acc
                })
                .into_iter()
                .rev()
                .collect(),
        )
    }
}

pub struct Navigation {
    lines: Vec<Line>,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Navigation {
    Navigation {
        lines: input
            .lines()
            .map(|l| {
                Line(
                    l.chars()
                        .map(|c| match c {
                            c if c == '(' || c == ')' => Chunk(P, c == '(', 3, 1),
                            c if c == '[' || c == ']' => Chunk(B, c == '[', 57, 2),
                            c if c == '{' || c == '}' => Chunk(C, c == '{', 1197, 3),
                            c if c == '<' || c == '>' => Chunk(A, c == '<', 25137, 4),
                            _ => unreachable!(),
                        })
                        .collect(),
                )
            })
            .collect(),
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &Navigation) -> usize {
    input
        .lines
        .iter()
        .filter_map(|l| {
            if let Some(c) = l.get_corrupted_chunk() {
                Some(c.2)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &Navigation) -> usize {
    let mut res = input
        .lines
        .iter()
        .filter_map(|l| {
            l.get_end()
                .map(|cs| cs.iter().fold(0, |acc, c| acc * 5 + c.3))
        })
        .collect::<Vec<_>>();
    res.sort_unstable();
    res[res.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            )),
            26397
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            )),
            288957
        )
    }
}
