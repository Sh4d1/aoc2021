#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<u64>) -> usize {
    input.windows(2).filter(|a| a[0] < a[1]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<u64>) -> usize {
    input.windows(4).filter(|a| a[0] < a[3]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "199
200
208
210
200
207
240
269
260
263"
            )),
            7
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "199
200
208
210
200
207
240
269
260
263"
            )),
            5
        )
    }

}
