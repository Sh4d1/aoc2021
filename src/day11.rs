use ndarray::Array2;

#[derive(Clone)]
pub struct Map {
    octo: Array2<(u64, u32)>,
}

impl Map {
    fn step(&mut self) {
        for o in self.octo.iter_mut() {
            o.0 += 1;
        }
        for ((x, y), o) in self.octo.clone().indexed_iter() {
            if o.0 > 9 {
                self.flash(x, y);
            }
        }
    }
    fn flash(&mut self, x: usize, y: usize) {
        let mut o = self.octo.get_mut((x, y)).unwrap();
        // Why is this if needed??
        if o.0 <= 9 {
            return;
        }
        o.1 += 1;
        o.0 = 0;
        for xi in std::cmp::max(0, x as i32 - 1) as usize
            ..=std::cmp::min(self.octo.shape()[0] as i32 - 1, x as i32 + 1) as usize
        {
            for yi in std::cmp::max(0, y as i32 - 1) as usize
                ..=std::cmp::min(self.octo.shape()[1] as i32 - 1, y as i32 + 1) as usize
            {
                let mut o = self.octo.get_mut((xi, yi)).unwrap();
                if (xi != x || yi != y) && o.0 != 0 {
                    o.0 += 1;
                    if o.0 > 9 {
                        self.flash(xi, yi);
                    }
                }
            }
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Map {
    let rows: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .collect::<String>()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();
    Map {
        octo: Array2::from_shape_fn((rows.len(), rows[0].len()), |(i, j)| (rows[i][j], 0)),
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &Map) -> usize {
    let input = &mut input.clone();
    for _ in 0..100 {
        input.step();
    }
    input.octo.iter().fold(0, |acc, o| acc + o.1 as usize)
}

#[aoc(day11, part2)]
pub fn part2(input: &Map) -> usize {
    let input = &mut input.clone();
    let mut i = 0;
    loop {
        input.step();
        i += 1;
        if input.octo.iter().all(|o| o.0 == 0) {
            break;
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            )),
            1656
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            )),
            195
        )
    }
}
