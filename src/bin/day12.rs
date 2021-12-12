use aoc2021::lines_as_vec;

use std::collections::{HashMap, HashSet};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let input = build_graph(&lines_as_vec("input/day12.txt"));

    find_all_paths(&input, &Cave::Start, &[]).len()
}

fn part2() -> usize {
    let input = build_graph(&lines_as_vec("input/day12.txt"));

    find_all_paths_two(&input, &Cave::Start, &[]).len()
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::Big(b) => write!(f, "{}", b),
            Cave::Small(b) => write!(f, "{}", b),
            Cave::Start => write!(f, "start"),
            Cave::End => write!(f, "end"),
        }
    }
}

impl From<&str> for Cave {
    fn from(name: &str) -> Self {
        if name == "start" {
            Cave::Start
        } else if name == "end" {
            Cave::End
        } else if name.chars().all(|c| c.is_lowercase()) {
            Cave::Small(name.to_string())
        } else {
            Cave::Big(name.to_string())
        }
    }
}

type CaveMap = HashMap<Cave, HashSet<Cave>>;

fn build_graph(pairs: &[String]) -> CaveMap {
    pairs.iter().fold(HashMap::new(), |mut map, p| {
        let (from, to) = p.split_once('-').unwrap();
        let from: Cave = from.into();
        let to: Cave = to.into();
        map.entry(from.clone())
            .or_insert_with(HashSet::new)
            .insert(to.clone());
        map.entry(to).or_insert_with(HashSet::new).insert(from);
        map
    })
}

fn find_all_paths(map: &CaveMap, here: &Cave, path: &[Cave]) -> Vec<Vec<Cave>> {
    let mut path = path.to_owned();
    path.push(here.clone());
    let mut paths = Vec::new();
    if *here == Cave::End {
        return vec![path];
    }
    for cave in map.get(here).unwrap() {
        let mut descend = |cave| {
            paths.append(&mut find_all_paths(map, cave, &path));
        };
        match cave {
            Cave::Big(_) | Cave::End => descend(cave),
            Cave::Small(_) => {
                if !path.contains(cave) {
                    descend(cave)
                }
            }
            Cave::Start => {}
        }
    }
    paths
}

fn find_all_paths_two(map: &CaveMap, here: &Cave, path: &[Cave]) -> Vec<Vec<Cave>> {
    let mut path = path.to_owned();
    path.push(here.clone());
    let mut paths = Vec::new();
    if *here == Cave::End {
        return vec![path];
    }
    for cave in map.get(here).unwrap() {
        let mut descend = |cave| {
            paths.append(&mut find_all_paths_two(map, cave, &path));
        };
        match cave {
            Cave::Big(_) | Cave::End => descend(cave),
            Cave::Small(_) => {
                let cave_cnt = path
                    .iter()
                    .filter(|&c| match c {
                        Cave::Small(_) | Cave::End => true,
                        Cave::Big(_) | Cave::Start => false,
                    })
                    .fold(HashMap::new(), |mut acc, c| {
                        *acc.entry(c).or_insert(0) += 1;
                        acc
                    });
                if let Some(&2) = cave_cnt.values().max() {
                    if cave_cnt.get(cave) == None {
                        descend(cave)
                    }
                } else {
                    descend(cave)
                }
            }
            Cave::Start => {} // skip
        }
    }
    paths
}

#[cfg(test)]
mod day12_tests {

    use super::*;
    use aoc2021::str_as_vec;

    #[test]
    fn test_find_all_paths_even_more() {
        let input = build_graph(&str_as_vec(
            "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
        ));

        let paths = find_all_paths(&input, &Cave::Start, &[]);
        assert_eq!(226, paths.len());
    }

    #[test]
    fn test_find_all_paths_more() {
        let input = build_graph(&str_as_vec(
            "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        ));

        let paths = find_all_paths(&input, &Cave::Start, &[]);
        assert_eq!(19, paths.len());
    }

    #[test]
    fn test_find_all_paths() {
        let input = build_graph(&str_as_vec(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        ));

        let paths = find_all_paths(&input, &Cave::Start, &[]);
        assert_eq!(10, paths.len());
    }

    #[test]
    fn test_find_all_paths_two() {
        let input = build_graph(&str_as_vec(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        ));

        let paths = find_all_paths_two(&input, &Cave::Start, &[]);
        for path in paths.iter() {
            println!("{:?}", path);
        }
        assert_eq!(36, paths.len());
    }

    #[test]
    fn test_build_graph() {
        let input = str_as_vec(
            "start-A
start-b
A-c",
        );

        let map = build_graph(&input);

        let result: CaveMap = [
            (
                Cave::Start,
                HashSet::from([Cave::Big("A".to_string()), Cave::Small("b".to_string())]),
            ),
            (
                Cave::Big("A".to_string()),
                HashSet::from([Cave::Small("c".to_string()), Cave::Start]),
            ),
            (Cave::Small("b".to_string()), HashSet::from([Cave::Start])),
            (
                Cave::Small("c".to_string()),
                HashSet::from([Cave::Big("A".to_string())]),
            ),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, map);
    }
}
