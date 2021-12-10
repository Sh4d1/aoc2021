#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Vec<char>]) -> usize {
    let mut res = 0;
    for i in input {
        let mut pushed = vec![];
        let mut corrupted = None;
        for j in i {
            match j {
                '{' => pushed.push(j),
                '(' => pushed.push(j),
                '[' => pushed.push(j),
                '<' => pushed.push(j),
                _ => {
                    let mut last = pushed.pop();
                    match last {
                        Some(&c) if c == '{' && *j != '}' => corrupted = Some(*j),
                        Some(&c) if c == '(' && *j != ')' => corrupted = Some(*j),
                        Some(&c) if c == '[' && *j != ']' => corrupted = Some(*j),
                        Some(&c) if c == '<' && *j != '>' => corrupted = Some(*j),
                        _ => (),
                    }
                }
            }
            if corrupted.is_some() {
                match corrupted.unwrap() {
                    '}' => res += 1197,
                    ']' => res += 57,
                    ')' => res += 3,
                    '>' => res += 25137,
                    _ => unreachable!(),
                }
                break;
            }
        }
    }
    res
}

#[aoc(day10, part2)]
pub fn part2(input: &[Vec<char>]) -> usize {
    let mut res = vec![];
    for i in input {
        let mut pushed = vec![];
        let mut corrupted = None;
        for j in i {
            match j {
                '{' => pushed.push(j),
                '(' => pushed.push(j),
                '[' => pushed.push(j),
                '<' => pushed.push(j),
                _ => {
                    let mut last = pushed.pop();
                    match last {
                        Some(&c) if c == '{' && *j != '}' => corrupted = Some(*j),
                        Some(&c) if c == '(' && *j != ')' => corrupted = Some(*j),
                        Some(&c) if c == '[' && *j != ']' => corrupted = Some(*j),
                        Some(&c) if c == '<' && *j != '>' => corrupted = Some(*j),
                        _ => (),
                    }
                }
            }
            if corrupted.is_some() {
                break;
            }
        }
        if corrupted.is_none() {
            res.push(pushed.iter().rev().fold(0, |mut acc, c| {
                acc *= 5;
                match c {
                    '(' => acc + 1,
                    '[' => acc + 2,
                    '{' => acc + 3,
                    '<' => acc + 4,
                    _ => acc,
                }
            }));
        }
    }
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
