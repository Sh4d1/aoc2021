fn most_least(input: &Vec<String>, i: usize) -> (char, char) {
    if 2 * input.iter().filter(|&s| s.chars().nth(i).unwrap() == '0').count() > input.len() {
        ('0', '1')
    } else {
        ('1', '0')
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    let mut gr = String::new();
    let mut er = String::new();
    
    for i in 0..(input[0].len()) {
        let ml = most_least(input, i);
        gr.push(ml.0);
        er.push(ml.1);
    }

    usize::from_str_radix(&gr, 2).unwrap() * usize::from_str_radix(&er, 2).unwrap()
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    let mut og = input.clone();
    let mut cs = input.clone();

    for i in 0..(input[0].len()) {
        if og.len() != 1 {
            let og_ml = most_least(&og, i);
            og.drain_filter(|s| { s.chars().nth(i).unwrap() != og_ml.0 });
        }
        if cs.len() != 1 {
            let cs_ml = most_least(&cs, i);
            cs.drain_filter(|s| { s.chars().nth(i).unwrap() == cs_ml.0 });
        }
        if og.len() == 1 && cs.len() == 1 {
            break;
        }
    }

    usize::from_str_radix(&og[0], 2).unwrap() * usize::from_str_radix(&cs[0], 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )),
            198
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )),
            230
        )
    }

}
