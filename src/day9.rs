use std::collections::HashSet;
use ndarray::Array2;

pub struct Heightmap {
    grid: Array2<u8>,
}

impl Heightmap {
    fn is_local_min(&self, x: usize, y: usize) -> bool {
        for xi in std::cmp::max(0, x as i32 - 1) as usize ..=std::cmp::min(self.grid.shape()[0] as i32 - 1, x as i32 + 1) as usize {
            for yi in std::cmp::max(0, y as i32 - 1) as usize..=std::cmp::min(self.grid.shape()[1] as i32 - 1, y as i32 + 1) as usize {
                if (xi == x || yi == y) && (xi != x || yi != y) {
                    if self.grid.get((x, y)).unwrap() >= self.grid.get((xi, yi)).unwrap() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn solve_1(&self) -> u32 {
        self.grid.indexed_iter().fold(0u32, |acc, ((x, y), &v)| {
            if self.is_local_min(x, y) {
                acc + v as u32 + 1
            } else {
                acc
            }
        })
    }

    fn n_higher(&self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> u32 {
        let mut res = 1;
        visited.insert((x, y));
        for xi in std::cmp::max(0, x as i32 - 1) as usize..=std::cmp::min(self.grid.shape()[0] as i32 - 1, x as i32 + 1) as usize {
            for yi in std::cmp::max(0, y as i32 - 1) as usize..=std::cmp::min(self.grid.shape()[1] as i32 - 1, y as i32 + 1) as usize {
                if (xi == x || yi == y) && (xi != x || yi != y) && self.grid.get((xi, yi)).unwrap() != &9 && !visited.contains(&(xi, yi)) {
                    res += self.n_higher(xi, yi, visited);
                }
            }
        }
        res
    }

    pub fn solve_2(&self) -> u32 {
        let mut basins = vec![];
        for ((x,y), _) in self.grid.indexed_iter() {
            if self.is_local_min(x, y) {
                basins.push(self.n_higher(x, y, &mut HashSet::new()));
            }
        }
        basins.sort_unstable_by(|a, b| b.cmp(a));
        basins.iter().take(3).fold(1, |acc, v| acc*v)
    }

}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Heightmap {
    let rows: Vec<Vec<u8>> = input.lines().map(|l| l.split_whitespace().collect::<String>().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect();
    Heightmap{grid: Array2::from_shape_fn((rows.len(),rows[0].len()), |(i,j)| rows[i][j])}
}

#[aoc(day9, part1)]
pub fn part1(input: &Heightmap) -> u32 {
    input.solve_1()
}

#[aoc(day9, part2)]
pub fn part2(input: &Heightmap) -> u32 {
    input.solve_2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            )),
            15
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "2199943210
3987894921
9856789892
8767896789
9899965678"
            )),
            1134
        )
    }

}
