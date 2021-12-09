use std::collections::HashMap;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines().map(
        |l| 
        {
            let mut split = l.split('|');
            let first = split.next().unwrap().split_whitespace().map(
                |s| 
                {
                    let mut a: Vec<_> = s.chars().collect();
                    a.sort_unstable();
                    a.iter().collect()
                }
            ).collect();
            let last = split.next().unwrap().split_whitespace().map(
                |s| 
                {
                    let mut a: Vec<_> = s.chars().collect();
                    a.sort_unstable();
                    a.iter().collect()
                }
            ).collect();
            (first, last)
        }
    ).collect()
}

pub fn get_n(input: &[String], output: &[String]) -> usize {
    let mut code = HashMap::new();
    let mut code_inv = HashMap::new();
    for i in input.iter().filter(|s| s.len() != 5 && s.len() != 6) {
        match i.len() {
            2 => {
                code.insert(i, 1);
                code_inv.insert(1, i);
            },
            3 => {
                code.insert(i, 7);
                code_inv.insert(7, i);
            },
            4 => {
                code.insert(i, 4);
                code_inv.insert(4, i);
            },
            7 => {
                code.insert(i, 8);
                code_inv.insert(8, i);
            },
            _ => unreachable!(),
        };
    }
    for i in input.iter().filter(|s| s.len() == 5 || s.len() == 6) {
        match i.len() {
            6 => {
                if code_inv.get(&4).unwrap().chars().all(|s| i.contains(s)) {
                    code.insert(i, 9);
                } else if code_inv.get(&1).unwrap().chars().all(|s| i.contains(s)) {
                    code.insert(i, 0);
                } else {
                    code.insert(i, 6);
                }
            },
            5 => {
                if code_inv.get(&7).unwrap().chars().all(|s| i.contains(s)) {
                    code.insert(i, 3);
                } else if code_inv.get(&4).unwrap().chars().filter(|s| i.contains(*s)).count() == 3  {
                    code.insert(i, 5);
                } else {
                    code.insert(i, 2);
                }
            },
            _ => unreachable!(),
        }
    }

    let mut res = 0;
    for (i, o) in output.iter().rev().enumerate() {
        res += (10usize.pow(i as u32)) * code.get(o).unwrap();
    }
    res
}

#[aoc(day8, part1)]
pub fn part1(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input.iter().fold(0, |acc, (_, o)| {
        acc + o.iter().fold(0, |acc, s| {
            if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 {
                acc + 1
            } else {
                acc
            }
        })
    })
}

#[aoc(day8, part2)]
pub fn part2(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input.iter().fold(0, |acc, (i,o)| {
        acc + get_n(i, o)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )),
            26
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )),
            61229
        )
    }

}
