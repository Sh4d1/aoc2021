#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Cell {
    East,
    South,
    Empty,
}
use Cell::*;

impl Copy for Cell {}

#[derive(Debug, Clone)]
pub struct Map {
    cells: Vec<Vec<Cell>>,
    h: usize,
    w: usize,
}

impl Map {
    fn step_east(&mut self) -> bool {
        let mut changed = false;
        let cells = self.cells.clone();
        for x in 0..self.w {
            for y in 0..self.h {
                // clippy wtf
                if cells[y as usize][x] == East {
                    let mut next_x = x + 1;
                    if next_x == self.w {
                        next_x = 0;
                    }
                    if cells[y][next_x] == Empty {
                        self.cells[y][next_x] = East;
                        self.cells[y][x] = Empty;
                        changed = true;
                    }
                }
            }
        }

        changed
    }
    fn step_south(&mut self) -> bool {
        let mut changed = false;
        let cells = self.cells.clone();
        for x in 0..self.w {
            for y in 0..self.h {
                if cells[y][x] == South {
                    let mut next_y = y + 1;
                    if next_y == self.h {
                        next_y = 0;
                    }
                    if cells[next_y][x] == Empty {
                        self.cells[next_y][x] = South;
                        self.cells[y][x] = Empty;
                        changed = true;
                    }
                }
            }
        }

        changed
    }
    fn step(&mut self) -> bool {
        self.step_east() | self.step_south()
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Map {
    let cells: Vec<Vec<Cell>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '>' => East,
                    'v' => South,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    Map {
        h: cells.len(),
        w: cells[0].len(),
        cells,
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &Map) -> usize {
    let mut input = input.clone();
    let mut i = 1;
    loop {
        if !input.step() {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
            )),
            58
        )
    }
}
