use ndarray::Array2;

#[derive(Debug)]
pub struct Bingo {
    draw: Vec<u64>,
    boards: Vec<Board>,
}

#[derive(Debug)]
pub struct Board {
    grid: Array2<(u64, bool)>,
    done: bool,
}

impl Bingo {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let draw: Vec<u64> = lines.next().unwrap().split(",").map(|d| d.parse().unwrap()).collect();
        lines.next();

        let lines: Vec<&str> = lines.collect::<Vec<&str>>().into_iter().filter(|l| !l.is_empty()).collect();
        Bingo{
            draw: draw,
            boards: lines.chunks(5).map(|l| {
                let rows: Vec<Vec<(u64, bool)>> = l.iter().map(|l| l.split_whitespace().map(|d| (d.parse().unwrap(), false)).collect()).collect();
                Board{
                    grid: Array2::from_shape_fn((rows[0].len(),rows.len()), |(j,i)| rows[j][i]),
                    done: false,
                }
            }).collect(),
        }
    }

    pub fn run1(&mut self) -> u64 {
        self.draw.clone().iter().find_map(
            |d| 
            self.boards.iter_mut()
                .find_map(|b| b.check(d))
        ).unwrap()
    }

    pub fn run2(&mut self) -> u64 {
        *self.draw.clone().iter().flat_map(
            |d| 
            self.boards.iter_mut()
                .filter(|b| !b.done)
                .filter_map(|b| b.check(d)).collect::<Vec<u64>>()
        ).collect::<Vec<u64>>().last().unwrap()
    }
}

impl Board {
    pub fn check(&mut self, v: &u64) -> Option<u64> {
        for (n, found) in self.grid.iter_mut() {
            if n == v {
                *found = true;
            }
        }
        if self.grid.rows().into_iter().any(|row| row.iter().all(|(_, found)| *found)) 
            || self.grid.columns().into_iter().any(|col| col.iter().all(|(_, found)| *found)) {
                self.done = true;
                return Some(v * self.grid.iter().filter(|(_, f)| !*f).map(|(n,_)| n).sum::<u64>());
        }
        None
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    Bingo::new(input).run1()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    Bingo::new(input).run2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            ),
            4512
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"

            ),
            1924
        )
    }

}
