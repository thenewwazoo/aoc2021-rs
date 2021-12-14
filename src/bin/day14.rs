use aoc2021::lines_as_vec;

use std::collections::HashMap;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let (mut s, m) = parse(&lines_as_vec("input/day14.txt"));

    for _ in 0..10 {
        s = do_insert(&s, &m);
    }
    count(&s)
}

fn part2() -> usize {
    let (mut s, m) = parse(&lines_as_vec("input/day14.txt"));

    for _ in 0..40 {
        s = do_insert(&s, &m);
    }
    count(&s)
}

type InsMap = HashMap<(char, char), char>;

fn parse(lines: &[String]) -> (Vec<char>, InsMap) {
    let start = lines[0].chars().collect();

    (
        start,
        HashMap::from_iter(lines.iter().skip(2).map(|l| {
            let c: Vec<char> = l.chars().collect();
            ((c[0], c[1]), c[6])
        })),
    )
}

fn do_insert(start: &[char], map: &InsMap) -> Vec<char> {
    start
        .windows(2)
        .fold(Vec::<char>::new(), |mut acc, w| {
            acc.push(w[0]);
            acc.push(*map.get(&(w[0], w[1])).unwrap());
            acc
        })
        .into_iter()
        .chain([*start.last().unwrap()].into_iter())
        .collect()
}

fn count(polymer: &[char]) -> usize {
    let counts = polymer.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let max = counts
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();
    let min = counts
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();
    counts[max] - counts[min]
}

#[cfg(test)]
mod day14_test {
    use super::*;
    use aoc2021::str_as_vec;

    const TEST_DATA: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_do_insert() {
        let (s, m) = parse(&str_as_vec(TEST_DATA));

        let s = do_insert(&s, &m);
        assert_eq!("NCNBCHB", String::from_iter(&s));
        let s = do_insert(&s, &m);
        assert_eq!("NBCCNBBBCBHCB", String::from_iter(&s));
        let s = do_insert(&s, &m);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", String::from_iter(&s));
        let s = do_insert(&s, &m);
        assert_eq!(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
            String::from_iter(&s)
        );
    }

    #[test]
    fn test_parse() {
        let (s, m) = parse(&str_as_vec(
            "NNCB

CH -> B",
        ));

        let s: String = s.iter().collect();
        assert_eq!("NNCB", s);
        assert_eq!(Some(&'B'), m.get(&('C', 'H')))
    }

    #[test]
    fn test_case() {
        let (mut s, m) = parse(&str_as_vec(TEST_DATA));

        for _ in 0..10 {
            s = do_insert(&s, &m);
        }
        assert_eq!(1588, count(&s));
    }
}
