use aoc2021::lines_as_vec;

use std::collections::HashMap;

// I'm going to leave the "hey buddy, did you forget the lesson from day 3?" solution I used for
// part 1 in there to prove that I, too, attempted to consume terabytes of ram on my laptop.

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let (mut s, m) = parse(&lines_as_vec("input/day14.txt"));

    for _ in 0..10 {
        s = do_insert(&s, &m);
    }
    count(&s)
}

fn part2() -> usize {
    let (start, m) = parse(&lines_as_vec("input/day14.txt"));
    let first = start[0];
    let mut pop = chunk_start(&start);

    for _ in 0..40 {
        pop = goddamned_lanternfish(pop, &m);
    }

    let counts = count_the_goddamned_fish(&pop, first);

    let max = counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k)
        .unwrap();
    let min = counts
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k)
        .unwrap();
    counts[max] - counts[min]
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
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k)
        .unwrap();
    let min = counts
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k)
        .unwrap();
    counts[max] - counts[min]
}

fn chunk_start(start: &[char]) -> HashMap<(char, char), usize> {
    start.windows(2).fold(HashMap::new(), |mut acc, w| {
        *acc.entry((w[0], w[1])).or_insert(0) += 1;
        acc
    })
}

fn goddamned_lanternfish(resmap: HashMap<(char, char), usize>, insmap: &InsMap) -> HashMap<(char, char), usize> {

    resmap.into_iter().fold(HashMap::new(), |mut acc, (p, n)| {
        let new = *insmap.get(&p).expect("bad pair");
        *acc.entry((p.0, new)).or_insert(0) += n;
        *acc.entry((new, p.1)).or_insert(0) += n;
        acc
    })
}

fn count_the_goddamned_fish(resmap: &HashMap<(char, char), usize>, first: char) -> HashMap<char, usize> {
    let mut r = resmap.iter().fold(HashMap::new(), |mut acc, (&(_, b), &c)| {
        *acc.entry(b).or_insert(0) += c;
        acc
    });
    *r.get_mut(&first).unwrap() += 1;
    r
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
    fn test_counting() {
        let s = "NBBBCNCCNBBNBNBBCHBHHBCHB";
        let start = chunk_start(&(s.chars().collect::<Vec<char>>()));
        let first = 'N';

        let counts = count_the_goddamned_fish(&start, first);

        let soln = HashMap::from([
            ('N', 5),
            ('B', 11),
            ('C', 5),
            ('H', 4)
        ]);
        assert_eq!(soln, counts);

        assert_eq!(s.len(), soln.iter().fold(0, |acc, (&_, &k)| acc + k));
    }


    #[test]
    fn test_goddamned_lanternfish() {
        let (s, m) = parse(&str_as_vec(TEST_DATA));

        let mut s = chunk_start(&s);

        println!("START {:?}\n", s);

        s = goddamned_lanternfish(s, &m);
        println!("DONE 1st {:?}\n", s);
        let t = chunk_start(&("NCNBCHB".chars().collect::<Vec<char>>()));
        assert_eq!(t, s);

        s = goddamned_lanternfish(s, &m);
        println!("DONE 2nd {:?}\n", s);
        let t = chunk_start(&("NBCCNBBBCBHCB".chars().collect::<Vec<char>>()));
        assert_eq!(t, s);

        s = goddamned_lanternfish(s, &m);
        println!("DONE 3rd {:?}\n", s);
        let t = chunk_start(&("NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<char>>()));
        assert_eq!(t, s);

        s = goddamned_lanternfish(s, &m);
        println!("DONE 4th {:?}\n", s);
        let t = chunk_start(&("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars().collect::<Vec<char>>()));
        assert_eq!(t, s);
    }

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

    #[test]
    fn test_goddamned_lanternfish_case() {
        let (start, m) = parse(&str_as_vec(TEST_DATA));
        let first = start[0];
        let mut pop = chunk_start(&start);

        for _ in 0..10 {
            pop = goddamned_lanternfish(pop, &m);
        }

        let counts = count_the_goddamned_fish(&pop, first);
        println!("{:?}", counts);

        let max = counts
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap();
        let min = counts
            .iter()
            .min_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap();
        assert_eq!(1588, counts[max] - counts[min]);

    }
}
