use std::collections::HashSet;
use ndarray::Array2;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Array2<u32> {
    let rows: Vec<Vec<u32>> = input.lines().map(|l| l.split_whitespace().collect::<String>().chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    Array2::from_shape_fn((rows[0].len(),rows.len()), |(j,i)| rows[i][j])
}

pub fn is_local_min(input: &Array2<u32>, x: usize, y: usize) -> bool {
    let v = input.get((x,y)).unwrap();
    let up = input.get((x, y+1)); 
    let mut down = None;
    if y as i32 - 1 >= 0 {
        down = input.get((x, y-1)); 
    }
    let mut left = None;
    if x as i32 - 1 >= 0 {
        left = input.get((x-1, y)); 
    }
    let right = input.get((x+1, y)); 
    if up.is_some() && v >= up.unwrap() {
        return false;
    }
    if down.is_some() && v >= down.unwrap() {
        return false;
    }
    if left.is_some() && v >= left.unwrap() {
        return false;
    }
    if right.is_some() && v >= right.unwrap() {
        return false;
    }

    true
}

#[aoc(day9, part1)]
pub fn part1(input: &Array2<u32>) -> u32 {
    input.indexed_iter().fold(0u32, |acc, ((x,y), &v)| {
        if is_local_min(input, x, y) {
            acc + v + 1
        } else {
            acc
        }
    })
}

pub fn n_higher(input: &Array2<u32>, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> u32 {
    let up = input.get((x, y+1)); 
    let mut down = None;
    if y as i32 - 1 >= 0 {
        down = input.get((x, y-1)); 
    }
    let mut left = None;
    if x as i32 - 1 >= 0 {
        left = input.get((x-1, y)); 
    }
    let right = input.get((x+1, y)); 
    visited.insert((x,y));

    let mut res = 1;
    if up.is_some() && up.unwrap() != &9 && !visited.contains(&(x, y+1)) {
        res += n_higher(input, x, y+1, visited);
    }
    if down.is_some() && down.unwrap() != &9 && !visited.contains(&(x, y-1)) {
        res += n_higher(input, x, y-1, visited);
    }
    if left.is_some() && left.unwrap() != &9 && !visited.contains(&(x-1, y)) {
        res += n_higher(input, x-1, y, visited);
    }
    if right.is_some() && right.unwrap() != &9 && !visited.contains(&(x+1, y)) {
        res += n_higher(input, x+1, y, visited);
    }

    res
}

#[aoc(day9, part2)]
pub fn part2(input: &Array2<u32>) -> u32 {
    let mut basins = vec![];
    for ((x,y), &v) in input.indexed_iter() {
        if is_local_min(input, x, y) {
            basins.push(n_higher(input, x, y, &mut HashSet::new()));
        }
    }
    basins.sort();
    basins.iter().rev().take(3).fold(1, |acc, v| acc*v)
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
