use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Polymer {
    rules: HashMap<(char, char), char>,
    count: HashMap<char, usize>,
    pair_count: HashMap<(char, char), usize>,
}

impl Polymer {
    fn step_n(&mut self, n: usize) -> usize {
        for _ in 0..n {
            let mut new_count = HashMap::new();
            for (p, c) in &self.pair_count {
                let to_insert = self.rules[p];
                *new_count.entry((p.0, to_insert)).or_default() += c;
                *new_count.entry((to_insert, p.1)).or_default() += c;
                *self.count.entry(to_insert).or_default() += c;
            }
            self.pair_count = new_count;
        }
        self.count.values().max().unwrap() - self.count.values().min().unwrap()
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Polymer {
    let mut lines = input.lines();
    let template: Vec<char> = lines.next().unwrap().chars().collect();
    let mut count = HashMap::new();
    let mut pair_count = HashMap::new();
    lines.next();

    let rules = lines.fold(HashMap::new(), |mut acc, l| {
        let (left, right) = l.split_once(" -> ").unwrap();
        acc.insert(
            (left.chars().next().unwrap(), left.chars().nth(1).unwrap()),
            right.chars().next().unwrap(),
        );
        acc
    });

    for c in template.iter() {
        *count.entry(*c).or_default() += 1;
    }
    for c in template.windows(2) {
        *pair_count.entry((c[0], c[1])).or_default() += 1;
    }

    Polymer {
        count,
        pair_count,
        rules,
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &Polymer) -> usize {
    let mut input = input.clone();
    input.step_n(10)
}

#[aoc(day14, part2)]
pub fn part2(input: &Polymer) -> usize {
    let mut input = input.clone();
    input.step_n(40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            )),
            1588
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            )),
            2188189693529
        )
    }
}
