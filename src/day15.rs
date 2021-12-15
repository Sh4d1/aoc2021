use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
pub struct Node(usize, usize, u64);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // works without chaining then_with hum
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(costs: &[Vec<u8>]) -> u64 {
    let width = costs[0].len();
    let height = costs.len();
    let end = (width - 1, height - 1);
    let mut distances = vec![vec![u64::MAX; width]; height];

    let mut queue = BinaryHeap::new();
    queue.push(Node(0, 0, 0));

    while let Some(Node(x, y, c)) = queue.pop() {
        if (x, y) == end {
            return c;
        }

        if c > distances[x][y] {
            continue;
        }

        for (xi, yi) in [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ] {
            if costs.get(xi).and_then(|r| r.get(yi)).is_none() {
                continue;
            }

            let new_cost = c + costs[xi][yi] as u64;

            if new_cost < distances[xi][yi] {
                distances[xi][yi] = new_cost;
                queue.push(Node(xi, yi, new_cost));
            }
        }
    }

    unreachable!()
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[Vec<u8>]) -> u64 {
    dijkstra(input)
}

#[aoc(day15, part2)]
pub fn part2(input: &[Vec<u8>]) -> u64 {
    let mut new_input = Vec::new();

    for y in 0..5 {
        for initial_row in input {
            let mut new_row = Vec::new();
            for x in 0..5 {
                new_row.extend(initial_row.iter().map(|c| match c + x + y {
                    n if n <= 9 => n,
                    n => n - 9,
                }));
            }
            new_input.push(new_row);
        }
    }
    dijkstra(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            )),
            40
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            )),
            315
        )
    }
}
