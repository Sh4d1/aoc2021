#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Pixel {
    Light,
    Dark,
}
use Pixel::*;

impl Copy for Pixel {}

#[derive(Clone)]
pub struct Image {
    pixels: Vec<Vec<Pixel>>,
    w: isize,
    h: isize,
    fill: Pixel,
}

impl Image {
    fn get_pixel(&self, x: isize, y: isize) -> Pixel {
        if x < 0 || y < 0 || x >= self.w || y >= self.h {
            self.fill
        } else {
            self.pixels[x as usize][y as usize]
        }
    }

    fn get_pixel_idx(&self, x: isize, y: isize) -> usize {
        ((x - 1)..=(x + 1))
            .flat_map(|xi| ((y - 1)..=(y + 1)).map(move |yi| self.get_pixel(xi, yi)))
            .rev()
            .enumerate()
            .map(|(i, p)| match p {
                Dark => 0,
                Light => 2usize.pow(i as u32),
            })
            .sum()
    }
}

#[derive(Clone)]
pub struct Input {
    algo: Vec<Pixel>,
    img: Image,
}

impl Input {
    fn enhance(&mut self) {
        self.img = Image {
            pixels: (-1..self.img.w + 1)
                .map(|x| {
                    (-1..self.img.h + 1)
                        .map(|y| self.algo[self.img.get_pixel_idx(x, y)])
                        .collect()
                })
                .collect(),
            w: self.img.w + 2,
            h: self.img.h + 2,
            fill: match self.img.fill {
                Dark => *self.algo.first().unwrap(),
                Light => *self.algo.last().unwrap(),
            },
        }
    }

    fn n_lit(&self) -> usize {
        self.img
            .pixels
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&p| *p == Light)
            .count()
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let algo = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '#' => Light,
            '.' => Dark,
            _ => unreachable!(),
        })
        .collect();
    lines.next();

    let pixels: Vec<Vec<Pixel>> = lines
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Light,
                    '.' => Dark,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    Input {
        algo,
        img: Image {
            w: pixels[0].len() as isize,
            h: pixels.len() as isize,
            pixels,
            fill: Dark,
        },
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    input.enhance();
    input.enhance();
    input.n_lit()
}

#[aoc(day20, part2)]
pub fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    for _ in 0..50 {
        input.enhance();
    }
    input.n_lit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(&input_generator("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###")), 35)
    }

    #[test]
    fn example2() {
        assert_eq!(part2(&input_generator("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###")), 3351)
    }
}
