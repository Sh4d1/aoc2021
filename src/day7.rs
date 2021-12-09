#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.split(',').map(|l| l.parse::<isize>().unwrap()).collect()
}

pub fn min_cost<F: Fn(isize, isize) -> isize>(input: &[isize], cost_fn: F) -> isize {
    let x_min = input.iter().min().unwrap();
    let x_max = input.iter().max().unwrap();

    (*x_min..=*x_max).map(
        |x_final| 
        input.iter().map(|&xi| cost_fn(x_final, xi)).sum()
    ).min().unwrap()
}

#[aoc(day7, part1)]
pub fn part1(input: &[isize]) -> isize {
    min_cost(input, 
        |x1, x2| 
        (x2 - x1).abs()
    )
}

#[aoc(day7, part2)]
pub fn part2(input: &[isize]) -> isize {
    min_cost(input, 
        |x1, x2| 
        {
            let d = (x2 - x1).abs();
            d * (d + 1) / 2
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "16,1,2,0,4,2,7,1,2,14"
            )),
            37
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "16,1,2,0,4,2,7,1,2,14"
            )),
            168
        )
    }

}
