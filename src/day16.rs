fn get_value(input: &[char]) -> usize {
    usize::from_str_radix(&input.iter().collect::<String>(), 2).unwrap()
}

fn parse(input: &[char]) -> (usize, usize, usize) {
    let version = get_value(&input[0..3]);
    let id = get_value(&input[3..6]);
    match id {
        4 => {
            let mut bin_res = vec![];
            let mut read = 0;
            for c in input[6..].chunks(5) {
                read += c.len();
                bin_res.append(&mut c[1..].to_owned());
                if let '0' = c[0] {
                    break;
                }
            }
            (version, 3 + 3 + read, get_value(&bin_res))
        }
        _ => {
            let mut read = 0;
            let mut versions = version;
            let mut values = vec![];
            match input[6] {
                '0' => {
                    let length = get_value(&input[7..7 + 15]);
                    let mut i = 7 + 15;
                    while read != length {
                        let (v, r, value) = parse(&input[i..]);
                        values.push(value);
                        i += r;
                        read += r;
                        versions += v;
                    }
                    read += 15;
                }
                '1' => {
                    let n = get_value(&input[7..7 + 11]);
                    let mut i = 7 + 11;
                    for _ in 0..n {
                        let (v, r, value) = parse(&input[i..]);
                        values.push(value);
                        i += r;
                        read += r;
                        versions += v;
                    }
                    read += 11;
                }
                _ => unreachable!(),
            }
            read += 3 + 3 + 1;
            match id {
                0 => (versions, read, values.iter().sum()),
                1 => (versions, read, values.iter().product()),
                2 => (versions, read, *values.iter().min().unwrap()),
                3 => (versions, read, *values.iter().max().unwrap()),
                5 => (versions, read, if values[0] > values[1] { 1 } else { 0 }),
                6 => (versions, read, if values[0] < values[1] { 1 } else { 0 }),
                7 => (versions, read, if values[0] == values[1] { 1 } else { 0 }),
                _ => unreachable!(),
            }
        }
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<char> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .fold(vec![], |mut acc, c| {
            match c {
                '0' => acc.append(&mut "0000".chars().collect()),
                '1' => acc.append(&mut "0001".chars().collect()),
                '2' => acc.append(&mut "0010".chars().collect()),
                '3' => acc.append(&mut "0011".chars().collect()),
                '4' => acc.append(&mut "0100".chars().collect()),
                '5' => acc.append(&mut "0101".chars().collect()),
                '6' => acc.append(&mut "0110".chars().collect()),
                '7' => acc.append(&mut "0111".chars().collect()),
                '8' => acc.append(&mut "1000".chars().collect()),
                '9' => acc.append(&mut "1001".chars().collect()),
                'A' => acc.append(&mut "1010".chars().collect()),
                'B' => acc.append(&mut "1011".chars().collect()),
                'C' => acc.append(&mut "1100".chars().collect()),
                'D' => acc.append(&mut "1101".chars().collect()),
                'E' => acc.append(&mut "1110".chars().collect()),
                'F' => acc.append(&mut "1111".chars().collect()),
                _ => unreachable!(),
            }
            acc
        })
}

#[aoc(day16, part1)]
pub fn part1(input: &[char]) -> usize {
    parse(input).0
}

#[aoc(day16, part2)]
pub fn part2(input: &[char]) -> usize {
    parse(input).2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator("A0016C880162017C3686B18A3D4780")),
            31
        )
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator("9C0141080250320F1802104A08")), 1)
    }
}
