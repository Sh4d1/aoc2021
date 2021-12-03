use std::collections::HashMap;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(
        |l|
        l.chars().map(
            |c|
            match c {
                '0' => 0,
                '1' => 1,
                _ => unreachable!(),
            }
        ).collect()
    ).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<u8>>) -> u64 {
    let mut res: Vec<i64> = vec![0; input[0].len()];
    for l in input.iter() {
        for it in l.iter().zip(res.iter_mut()) {
            match it.0 {
                0 => *it.1 -=1,
                1 => *it.1 +=1,
                _ => unreachable!(),
            } 
        }
    }

    let mut f1 = 0;
    let mut f2 = 0;
    for (i, r) in res.iter().enumerate() {
        match r {
            d if *r > 0 => f1 += 2u64.pow((res.len()-i-1) as u32),
            d if *r < 0 => f2 += 2u64.pow((res.len()-i-1) as u32),
            _ => (),
        }
    }
    f1*f2

}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<u8>>) -> u64 {
    let mut og_map = HashMap::new();
    let mut cs_map = HashMap::new();
    for (i, l) in input.iter().enumerate() {
        og_map.insert(i, l);
        cs_map.insert(i, l);
    }

    for i in 0..(input[0].len()) {
        let mut res_og = 0;
        let mut res_cs = 0;
        for (i1, l1) in input.iter().enumerate() {
            match l1[i] {
                0 => {
                    if og_map.contains_key(&i1) {
                        res_og -= 1
                    }
                    if cs_map.contains_key(&i1) {
                        res_cs -= 1
                    }
                },
                1 => {
                    if og_map.contains_key(&i1) {
                        res_og += 1
                    }
                    if cs_map.contains_key(&i1) {
                        res_cs += 1
                    }
                },
                _ => unreachable!(),
            } 
        }

        for (j, l) in input.iter().enumerate() {
            if og_map.contains_key(&j) && og_map.len() > 1 {
                if res_og >= 0 && l[i] == 0 || res_og < 0 && l[i] == 1 {
                    og_map.remove(&j);
                }
            }
            if cs_map.contains_key(&j) && cs_map.len() > 1{
                if res_cs >= 0 && l[i] == 1 || res_cs < 0 && l[i] == 0 {
                    cs_map.remove(&j);
                }
            }
        }

        if cs_map.len() == 1 && og_map.len() == 1 {
            break;
        }
    }

    let mut og = 0;
    let mut cs = 0;
    for (i, r) in (og_map.into_values().next().unwrap()).iter().enumerate() {
        match r {
            d if *r > 0 => og += 2u64.pow((input[0].len()-i-1) as u32),
            _ => (),
        }
    }


    for (i, r) in (cs_map.into_values().next().unwrap()).iter().enumerate() {
        match r {
            d if *r > 0 => cs += 2u64.pow((input[0].len()-i-1) as u32),
            _ => (),
        }
    }

    og*cs
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
