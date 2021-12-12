use std::collections::HashMap;
use std::collections::HashSet;

pub struct Map {
    nodes: HashMap<Type, Node>,
}

impl Map {
    fn visit_all_from(&self, t: Type, current: Vec<Type>, s: bool) -> Vec<Vec<Type>> {
        let start = self.nodes.get(&t).unwrap().clone();
        let mut res = vec![];

        for n in start.edges {
            let mut new_s = s;
            match n {
                Type::Start => continue,
                Type::End => {
                    res.push(current.clone());
                    continue;
                }
                Type::Small(_) if current.contains(&n) => {
                    if !s {
                        new_s = true;
                    } else {
                        continue;
                    }
                }
                _ => (),
            }
            let mut new_current = current.clone();
            new_current.push(n.clone());
            res.append(&mut self.visit_all_from(n, new_current, new_s))
        }
        res
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Type {
    Start,
    End,
    Small(String),
    Big(String),
}

#[derive(Clone)]
pub struct Node {
    edges: HashSet<Type>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Map {
    let mut nodes: HashMap<Type, Node> = HashMap::new();
    input.lines().for_each(|l| {
        let nodes_type: Vec<Type> = l
            .split('-')
            .map(|w| match w {
                "start" => Type::Start,
                "end" => Type::End,
                x if x.chars().all(|c| c.is_ascii_uppercase()) => Type::Big(x.to_owned()),
                x => Type::Small(x.to_owned()),
            })
            .collect();
        nodes
            .entry(nodes_type[0].clone())
            .or_insert(Node {
                edges: HashSet::new(),
            })
            .edges
            .insert(nodes_type[1].clone());
        nodes
            .entry(nodes_type[1].clone())
            .or_insert(Node {
                edges: HashSet::new(),
            })
            .edges
            .insert(nodes_type[0].clone());
    });
    Map { nodes }
}

#[aoc(day12, part1)]
pub fn part1(input: &Map) -> usize {
    input.visit_all_from(Type::Start, vec![], true).len()
}

#[aoc(day12, part2)]
pub fn part2(input: &Map) -> usize {
    input.visit_all_from(Type::Start, vec![], false).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            part1(&input_generator(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            )),
            10
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            part2(&input_generator(
                "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            )),
            36
        )
    }
}