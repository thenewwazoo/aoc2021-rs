use aoc2021::lines_as_vec;

use std::collections::HashSet;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let (mut map, instrs) = parse_input(&lines_as_vec("input/day13.txt"));

    fold(&mut map, &instrs[0]);

    map.len()
}

fn part2() -> String {
    let (mut map, instrs) = parse_input(&lines_as_vec("input/day13.txt"));

    for instr in instrs {
        fold(&mut map, &instr);
    }

    format!("{}", MapPrinter(map))
}

#[derive(Debug, Eq, PartialEq)]
enum Fold {
    Horiz(usize),
    Vert(usize),
}

type Map = HashSet<(usize, usize)>;

struct MapPrinter(Map);

impl std::fmt::Display for MapPrinter {
    fn fmt(self: &MapPrinter, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_max, y_max) = find_extents(&self.0);
        for y in 0..=y_max {
            for x in 0..=x_max {
                if matches!(self.0.get(&(x, y)), Some(_)) {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

fn parse_input(input: &[String]) -> (Map, Vec<Fold>) {
    let mut input = input.iter();

    let mut map = HashSet::new();
    while let Some((x, y)) = match input.next() {
        Some(v) => v.split_once(','),
        None => None,
    } {
        map.insert((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }

    let mut instr = Vec::new();
    for s in input {
        let (words, offset) = s.split_once('=').unwrap();
        instr.push(match words.chars().last().unwrap() {
            'x' => Fold::Vert(offset.parse::<usize>().unwrap()),
            'y' => Fold::Horiz(offset.parse::<usize>().unwrap()),
            _ => panic!("bad spec"),
        });
    }
    (map, instr)
}

fn find_extents(map: &Map) -> (usize, usize) {
    map.iter()
        .fold((0, 0), |acc, &(x, y)| (x.max(acc.0), y.max(acc.1)))
}

fn fold(map: &mut Map, fold: &Fold) {
    match *fold {
        Fold::Horiz(y) => fold_horiz(map, y),
        Fold::Vert(x) => fold_vert(map, x),
    }
}

fn fold_vert(map: &mut Map, crease: usize) {
    let (x_max, y_max) = find_extents(map);

    for y in 0..=y_max {
        map.remove(&(crease, y));
    }

    for x_off in 1..=(x_max - crease) {
        for y in 0..=y_max {
            if map.remove(&(crease + x_off, y)) {
                map.insert((crease - x_off, y));
            }
        }
    }
}

fn fold_horiz(map: &mut Map, crease: usize) {
    let (x_max, y_max) = find_extents(map);

    for x in 0..=x_max {
        map.remove(&(x, crease));
    }

    for y_off in 0..=(y_max - crease) {
        for x in 0..=x_max {
            if map.remove(&(x, crease + y_off)) {
                map.insert((x, crease - y_off));
            }
        }
    }
}

#[cfg(test)]
mod day13_tests {

    use aoc2021::str_as_vec;

    use super::*;

    #[test]
    fn test_fold() {
        let (mut map, _) = parse_input(&str_as_vec("2,2"));

        fold(&mut map, &Fold::Horiz(1));
        fold(&mut map, &Fold::Vert(1));

        assert_eq!(map, parse_input(&str_as_vec("0,0")).0);
    }

    #[test]
    fn test_fold_vert() {
        let (mut map, _) = parse_input(&str_as_vec(
            "2,0
0,1
2,2",
        ));

        fold(&mut map, &Fold::Vert(1));

        assert_eq!(
            map,
            parse_input(&str_as_vec(
                "0,0
0,1
0,2"
            ))
            .0
        );
    }

    #[test]
    fn test_fold_horiz() {
        let (mut map, _) = parse_input(&str_as_vec(
            "0,2
1,0
2,2",
        ));

        fold(&mut map, &Fold::Horiz(1));

        assert_eq!(
            map,
            parse_input(&str_as_vec(
                "0,0
1,0
2,0"
            ))
            .0
        );
    }

    #[test]
    fn test_parse() {
        let (m, i) = parse_input(&str_as_vec(
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
fold along x=5",
        ));

        assert!(matches!(m.get(&(0, 14)), Some(_)));
        assert_eq!(i[0], Fold::Horiz(7));
    }

    #[test]
    fn test_case() {
        let (mut map, i) = parse_input(&str_as_vec(
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
fold along x=5",
        ));

        for instr in i {
            fold(&mut map, &instr);
        }

        //assert_eq!(17, map.0.len()); // writeup says 17, that's a bug
        assert_eq!(16, map.len());
    }
}
