use aoc2021::lines_as_vec;

use std::collections::HashMap;

fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> usize {
    let lines = lines_as_vec("input/day5.txt");
    let mut map = HashMap::new();

    lines
        .iter()
        .map(|l| parse_line(l))
        .for_each(|t| record_seg(t.0 .0, t.0 .1, t.1 .0, t.1 .1, &mut map));

    map.values().filter(|&&v| v > 1).cloned().count()
}

fn parse_line(line: &str) -> ((u64, u64), (u64, u64)) {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let p1: Vec<&str> = parts[0].split(',').collect();
    let p2: Vec<&str> = parts[1].split(',').collect();

    (
        (p1[0].parse::<u64>().unwrap(), p1[1].parse::<u64>().unwrap()),
        (p2[0].parse::<u64>().unwrap(), p2[1].parse::<u64>().unwrap()),
    )
}

fn record_seg(x1: u64, y1: u64, x2: u64, y2: u64, map: &mut HashMap<(u64, u64), u64>) {
    if x1 == x2 {
        if y1 < y2 { y1..=y2 } else { y2..=y1 }.for_each(|y| {
            map.entry((x1, y)).and_modify(|v| *v += 1).or_insert(1);
        })
    } else if y1 == y2 {
        if x1 < x2 { x1..=x2 } else { x2..=x1 }.for_each(|x| {
            map.entry((x, y1)).and_modify(|v| *v += 1).or_insert(1);
        })
    } else {
        let x: Vec<u64> = if x1 < x2 {
            (x1..=x2).collect()
        } else {
            (x2..=x1).rev().collect()
        };
        let y: Vec<u64> = if y1 < y2 {
            (y1..=y2).collect()
        } else {
            (y2..=y1).rev().collect()
        };

        x.into_iter().zip(y.into_iter()).for_each(|(x, y)| {
            map.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
        })
    }
}

#[cfg(test)]
mod day5_tests {

    use super::*;
    use aoc2021::str_as_vec;

    #[test]
    fn test_parse_line() {
        let line = "0,9 -> 5,9";
        assert_eq!(parse_line(line), ((0, 9), (5, 9)));
    }

    #[test]
    fn test_record_seg() {
        let mut map = HashMap::new();

        record_seg(1, 1, 1, 3, &mut map);

        assert_eq!(map.get(&(1, 1)), Some(&1));
        assert_eq!(map.get(&(1, 2)), Some(&1));
        assert_eq!(map.get(&(1, 3)), Some(&1));

        let mut map = HashMap::new();
        record_seg(9, 7, 7, 7, &mut map);

        assert_eq!(map.get(&(9, 7)), Some(&1));
        assert_eq!(map.get(&(8, 7)), Some(&1));
        assert_eq!(map.get(&(7, 7)), Some(&1));

        let mut map = HashMap::new();
        record_seg(1, 1, 3, 3, &mut map);

        assert_eq!(map.get(&(1, 1)), Some(&1));
        assert_eq!(map.get(&(2, 2)), Some(&1));
        assert_eq!(map.get(&(3, 3)), Some(&1));

        let mut map = HashMap::new();
        record_seg(9, 7, 7, 9, &mut map);

        assert_eq!(map.get(&(9, 7)), Some(&1));
        assert_eq!(map.get(&(8, 8)), Some(&1));
        assert_eq!(map.get(&(7, 9)), Some(&1));
    }

    #[test]
    fn test_case() {
        let lines = str_as_vec(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        );

        let mut map = HashMap::new();

        lines
            .iter()
            .map(|l| parse_line(l))
            .for_each(|t| record_seg(t.0 .0, t.0 .1, t.1 .0, t.1 .1, &mut map));

        println!("{:?}", map);
        assert_eq!(map.values().filter(|&&v| v > 1).cloned().count(), 12);
    }
}
