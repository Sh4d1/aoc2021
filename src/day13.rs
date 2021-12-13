use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Fold {
    X(u64),
    Y(u64),
}

#[derive(Clone, Debug)]
pub struct Paper {
    points: HashSet<(u64, u64)>,
    folds: Vec<Fold>,
}

impl Paper {
    fn fold(&mut self) -> bool {
        if self.folds.is_empty() {
            return false;
        }
        let fold = self.folds.remove(0);
        let points = self
            .points
            .iter()
            .map(|&(x, y)| match fold {
                Fold::X(r) if x < r => (x, y),
                Fold::X(r) => (2 * r - x, y),
                Fold::Y(r) if y < r => (x, y),
                Fold::Y(r) => (x, 2 * r - y),
            })
            .collect();
        self.points = points;
        true
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Paper {
    input.lines().fold(
        Paper {
            points: HashSet::new(),
            folds: vec![],
        },
        |mut acc, l| {
            if l.starts_with("fold along x=") {
                let (_, fold_x) = l.split_once("fold along x=").unwrap();
                acc.folds.push(Fold::X(fold_x.parse().unwrap()));
            } else if l.starts_with("fold along y=") {
                let (_, fold_y) = l.split_once("fold along y=").unwrap();
                acc.folds.push(Fold::Y(fold_y.parse().unwrap()));
            } else if l.is_empty() {
                return acc;
            } else {
                let (x, y) = l.split_once(',').unwrap();
                acc.points.insert((x.parse().unwrap(), y.parse().unwrap()));
            }
            acc
        },
    )
}

#[aoc(day13, part1)]
pub fn part1(input: &Paper) -> usize {
    let paper = &mut input.clone();
    paper.fold();
    paper.points.len()
}

#[aoc(day13, part2)]
pub fn part2(input: &Paper) -> usize {
    let paper = &mut input.clone();
    while paper.fold() {}

    // huh no easy way to get a String here ?

    let max_x = paper.points.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = paper.points.iter().map(|&(_, y)| y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.points.contains(&(x, y)) {
                print!("88");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            )),
            17
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            )),
            0
        )
    }
}
