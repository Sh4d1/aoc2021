use std::collections::HashMap;

const P1_SCORE: usize = 1000;
const P2_SCORE: usize = 21;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Player {
    pos: usize,
    score: usize,
}

impl Copy for Player {}

impl Player {
    fn play(self, d: usize) -> Player {
        let mut p = Player {
            score: self.score,
            pos: (self.pos + d) % 10,
        };
        if p.pos == 0 {
            p.pos = 10;
        }
        p.score += p.pos;
        p
    }
}

fn play1<I>(mut p: Player, p_idle: Player, mut roll: I) -> usize
where
    I: Iterator<Item = (usize, usize)>,
{
    let r1 = roll.next().unwrap();
    let r2 = roll.next().unwrap();
    let r3 = roll.next().unwrap();
    p = p.play(r1.1 + r2.1 + r3.1);
    if p.score >= P1_SCORE {
        return (r3.0 + 1) * p_idle.score;
    }
    play1(p_idle, p, roll)
}

fn play2(
    p: Player,
    p_idle: Player,
    hm: &mut HashMap<(Player, Player), (usize, usize)>,
) -> (usize, usize) {
    if let Some(s) = hm.get(&(p, p_idle)) {
        return *s;
    }

    let mut s = (0, 0);

    for (r, n) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let new_p = Player {
            pos: p.pos,
            score: p.score,
        }
        .play(r);
        if new_p.score >= P2_SCORE {
            s.0 += n;
        } else {
            let (s1, s2) = play2(p_idle, new_p, hm);
            s = (s.0 + s2 * n, s.1 + s1 * n);
        }
    }
    hm.insert((p, p_idle), s);
    s
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (Player, Player) {
    let mut lines = input.lines();
    let (_, sp1) = lines.next().unwrap().split_once(": ").unwrap();
    let (_, sp2) = lines.next().unwrap().split_once(": ").unwrap();
    (
        Player {
            pos: sp1.parse().unwrap(),
            score: 0,
        },
        Player {
            pos: sp2.parse().unwrap(),
            score: 0,
        },
    )
}

#[aoc(day21, part1)]
pub fn part1(input: &(Player, Player)) -> usize {
    play1(input.0, input.1, (1..=100).cycle().enumerate())
}

#[aoc(day21, part2)]
pub fn part2(input: &(Player, Player)) -> usize {
    let (p1, p2) = play2(input.0, input.1, &mut HashMap::new());
    p1.max(p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "Player 1 starting position: 4
Player 2 starting position: 8"
            )),
            739785
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "Player 1 starting position: 4
Player 2 starting position: 8"
            )),
            444356092776315
        )
    }
}
