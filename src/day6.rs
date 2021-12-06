use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> HashMap<u64, u64> {
    input.lines().next().unwrap().split(",").fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c.parse::<u64>().unwrap()).or_insert(0) += 1;
        acc
    })
}

fn simulate(input: HashMap<u64, u64>, n: u64) -> u64 {
    let mut input = input;
    for _ in 0..n {
        input = input.iter().fold(HashMap::new(), |mut acc, (&s, n)| {
            if s == 0 {
                *acc.entry(8).or_insert(0) += n;
                *acc.entry(6).or_insert(0) += n;
            } else {
                *acc.entry(s-1).or_insert(0) += n;
            }
            acc
        });
    }
    input.iter().fold(0, |acc, (_, &n)| acc+n)
}

#[aoc(day6, part1)]
pub fn part1(input: &HashMap<u64, u64>) -> u64 {
    simulate(input.clone(), 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &HashMap<u64, u64>) -> u64 {
    simulate(input.clone(), 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "3,4,3,1,2"
            )),
            5934
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "3,4,3,1,2"
            )),
            26984457539
        )
    }

}
