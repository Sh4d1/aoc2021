use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const Y_ROOM_START: usize = 2;
const Y_HALLWAY: usize = 1;

const X_RANGE_HALLWAY: std::ops::Range<usize> = 1..12;
const X_RANGE_FINAL: std::ops::Range<usize> = 3..10;

lazy_static! {
    static ref FINAL_POS: HashMap<char, (usize, usize)> = {
        let mut hm = HashMap::new();
        hm.insert('A', (3, 1));
        hm.insert('B', (5, 10));
        hm.insert('C', (7, 100));
        hm.insert('D', (9, 1000));
        hm
    };
}

#[derive(Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    map: Vec<Vec<char>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn is_final(entry: &[Vec<char>]) -> bool {
    entry[Y_ROOM_START..(entry.len() - 1)]
        .iter()
        .all(|l| l[X_RANGE_FINAL].iter().collect::<String>() == "A#B#C#D")
}

fn get_possible_moves(entry: &[Vec<char>]) -> Vec<(Vec<Vec<char>>, usize)> {
    let mut pmoves = vec![];
    let room_size = entry.len() - 2;

    for x in X_RANGE_HALLWAY {
        match entry[Y_HALLWAY][x] {
            '.' => continue,
            _ => {
                let (idx, energy) = FINAL_POS.get(&entry[Y_HALLWAY][x]).unwrap().to_owned();
                let mut x_range = idx..x;
                if x <= idx {
                    x_range = x + 1..idx + 1;
                }

                if x_range.any(|xi| entry[Y_HALLWAY][xi] != '.') {
                    // obstacle
                    continue;
                }

                let last_empty;
                if let Some(i) = (Y_ROOM_START..=room_size)
                    .take_while(|&y| entry[y][idx] == '.')
                    .last()
                {
                    last_empty = i;
                } else {
                    // no space in room
                    continue;
                }

                if last_empty != room_size
                    && (last_empty + 1..=room_size).any(|yi| entry[yi][idx] != entry[Y_HALLWAY][x])
                {
                    // not correct value in room or full
                    continue;
                }

                let mut new_move = entry.to_owned();
                new_move[last_empty][idx] = entry[Y_HALLWAY][x];
                new_move[Y_HALLWAY][x] = '.';
                pmoves.push((
                    new_move,
                    (last_empty - 1 + (x as isize - idx as isize).abs() as usize) * energy,
                ));
            }
        }
    }
    for y in Y_ROOM_START..=room_size {
        for x in FINAL_POS.iter().map(|(_, v)| v.0) {
            if entry[y][x] == '.'
                || (Y_ROOM_START..y).any(|yi| entry[yi][x] != '.')
                || (y + 1..=room_size).any(|yi| entry[yi][x] == '.')
            {
                continue;
            }

            let (_, energy) = FINAL_POS.get(&entry[y][x]).unwrap().to_owned();
            pmoves.append(
                &mut (x..entry[0].len())
                    .take_while(|&xi| entry[Y_HALLWAY][xi] == '.')
                    .filter(|&xi| FINAL_POS.iter().all(|(_, v)| v.0 != xi))
                    .map(|xi| {
                        let mut new_move = entry.to_owned();
                        new_move[Y_HALLWAY][xi] = entry[y][x];
                        new_move[y][x] = '.';
                        (new_move, (y - 1 + xi - x) * energy)
                    })
                    .collect(),
            );
            pmoves.append(
                &mut (1..=x)
                    .rev()
                    .take_while(|&xi| entry[Y_HALLWAY][xi] == '.')
                    .filter(|&xi| FINAL_POS.iter().all(|(_, v)| v.0 != xi))
                    .map(|xi| {
                        let mut new_move = entry.to_owned();
                        new_move[Y_HALLWAY][xi] = entry[y][x];
                        new_move[y][x] = '.';
                        (new_move, (y - 1 + x - xi) * energy)
                    })
                    .collect(),
            )
        }
    }
    pmoves
}

fn shortest_path_cost(i: &[Vec<char>]) -> usize {
    let mut h = BinaryHeap::new();
    let mut d = HashMap::new();
    h.push(State {
        cost: 0,
        map: i.to_vec(),
    });
    while let Some(s) = h.pop() {
        if is_final(&s.map) {
            return s.cost;
        }

        if let Some(&cost) = d.get(&s.map) {
            if s.cost > cost {
                continue;
            }
        }

        for (m, n) in get_possible_moves(&s.map) {
            let new_c = s.cost + n;
            let current_c = d.get(&m).unwrap_or(&usize::MAX);
            if new_c < *current_c {
                d.insert(m.clone(), new_c);
                h.push(State {
                    cost: new_c,
                    map: m,
                })
            }
        }
    }
    unreachable!();
}

#[aoc(day23, part1)]
pub fn part1(input: &[Vec<char>]) -> usize {
    shortest_path_cost(input)
}

#[aoc(day23, part2)]
pub fn part2(input: &[Vec<char>]) -> usize {
    let mut input = input.to_vec();
    input.insert(3, "  #D#B#A#C#".chars().collect());
    input.insert(3, "  #D#C#B#A#".chars().collect());
    shortest_path_cost(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            )),
            12521
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            )),
            44169
        )
    }
}
