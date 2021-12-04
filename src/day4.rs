use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Board {
    init: Vec<Vec<u64>>,
    possibles: Vec<Vec<u64>>,
}

#[derive(Debug, Clone)]
pub struct Bingo {
    draw: Vec<u64>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Bingo {
    let mut lines = input.lines();
    let mut boards: Vec<Board> = Vec::new();
    let draw = lines.next().unwrap().split(",").map(|d| d.parse::<u64>().unwrap()).collect();
    lines.next();
    let mut line = lines.next();
    let mut tmp = vec![];
    loop {
        if line.is_none() || line.unwrap().trim().is_empty() {
            let mut b = Board{init: tmp, possibles: vec![]};
            let mut columns: Vec<Vec<u64>> = vec![vec![]; b.init[0].len()];
            for (i, d) in b.init.iter().enumerate() {
                for (j, e) in d.iter().enumerate() {
                    columns[j].push(*e); 
                }
                b.possibles.push(d.to_vec());
            }
            b.possibles.append(&mut columns);

            boards.push(b);
            tmp = vec![];
            if line.is_none() {
                break;
            }
        } else {
            let numbers = line.unwrap().split_whitespace().map(|d| d.parse::<u64>().unwrap()).collect(); 
            tmp.push(numbers);
        }
        line = lines.next();
    }
    Bingo{
        draw: draw,
        boards: boards,
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &Bingo) -> u64 {
    let mut i = (*input).clone();
    let mut win = 0;
    let mut res = 0;
    for n in input.draw.iter() {
        for b in i.boards.iter_mut() {
            for p in b.possibles.iter_mut() {
                p.retain(|&x| x != *n);
                if p.is_empty(){
                    win = *n;
                }
            }
            if win != 0 {
                let mut hs = HashSet::new();
                for p in b.possibles.iter() {
                    for n in p.iter() {
                        if !hs.contains(n) {
                            res += n;
                            hs.insert(n);
                        }
                    }
                }
                break;
            }
        }
        if win != 0 {
            break;
        }
    }
    res*win
}

#[aoc(day4, part2)]
pub fn part2(input: &Bingo) -> u64 {
    let mut i = (*input).clone();
    let mut win = 0;
    let mut res = 0;
    let mut n_wins = HashSet::new();
    for n in input.draw.iter() {
        for (j, b) in i.boards.iter_mut().enumerate() {
            for p in b.possibles.iter_mut() {
                p.retain(|&x| x != *n);
                if p.is_empty(){
                    win = *n;
                    n_wins.insert(j);
                }
            }
            if win != 0 && n_wins.len() == input.boards.len() {
                let mut hs = HashSet::new();
                for p in b.possibles.iter() {
                    for n in p.iter() {
                        if !hs.contains(n) {
                            res += n;
                            hs.insert(n);
                        }
                    }
                }
                break;
            }
        }
        if n_wins.len() == input.boards.len() {
            break;
        }
    }
    res*win
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
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
            )),
            4512
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
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

            )),
            1924
        )
    }

}
