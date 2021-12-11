use aoc2021::lines_as_vec;

use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let map = lines_to_map(&lines_as_vec("input/day9.txt"));
    map.iter()
        .map(|(&(x, y), &c)| (c, is_low(&map, x, y))) // turn every k,v into (v, ?low)
        .filter(|(_, l)| *l) // only keep the low
        .fold(0, |acc, (c, _)| acc + c.to_digit(10).unwrap() + 1) // add up all the v
}

fn part2() -> usize {
    let map = lines_to_map(&lines_as_vec("input/day9.txt"));
    let tmap = tag_basin(&map);

    let bsz = count_basin_sz(&tmap);

    let mut v: Vec<usize> = bsz.values().cloned().collect();
    v.sort_unstable();
    v.into_iter()
        .rev()
        .take(3)
        .into_iter()
        .reduce(|acc, v| acc * v)
        .unwrap()
}

fn lines_to_map(data: &[String]) -> HashMap<(isize, isize), char> {
    let width = data[0].len() as isize;
    data.iter()
        .flat_map(|l| l.chars())
        .enumerate()
        .map(|(i, c)| ((i as isize % width, i as isize / width), c))
        .collect()
}

fn is_low(map: &HashMap<(isize, isize), char>, x: isize, y: isize) -> bool {
    let ct = *map.get(&(x, y)).expect("center not in map");

    let up = map.get(&(x, y - 1));
    let dn = map.get(&(x, y + 1));
    let lt = map.get(&(x - 1, y));
    let rt = map.get(&(x + 1, y));

    let mut cares = Vec::new();
    if let Some(&u) = up {
        cares.push(u > ct);
    }
    if let Some(&d) = dn {
        cares.push(d > ct);
    }
    if let Some(&l) = lt {
        cares.push(l > ct);
    }
    if let Some(&r) = rt {
        cares.push(r > ct);
    }

    cares.iter().all(|&c| c)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BasinTag {
    Unknown,
    Basin(usize),
    Ridge,
}

fn find_next(map: &HashMap<(isize, isize), BasinTag>) -> Option<(isize, isize)> {
    map.iter()
        .find(|&(_, &c)| c == BasinTag::Unknown)
        .map(|(&(x, y), _)| (x, y))
}

fn tag_basin(map: &HashMap<(isize, isize), char>) -> HashMap<(isize, isize), BasinTag> {
    let mut bmap: HashMap<(isize, isize), BasinTag> = map
        .iter()
        .map(|(&(x, y), &c)| {
            (
                (x, y),
                match c {
                    '9' => BasinTag::Ridge,
                    _ => BasinTag::Unknown,
                },
            )
        })
        .collect();

    let mut color = 0;

    while let Some(node) = find_next(&bmap) {
        let mut fill_stack: Vec<(isize, isize)> = vec![node];
        while let Some(n) = fill_stack.pop() {
            match bmap.get(&n) {
                Some(BasinTag::Unknown) => {
                    bmap.insert(n, BasinTag::Basin(color));
                    fill_stack.push((n.0 + 1, n.1));
                    fill_stack.push((n.0 - 1, n.1));
                    fill_stack.push((n.0, n.1 + 1));
                    fill_stack.push((n.0, n.1 - 1));
                }
                Some(BasinTag::Basin(c)) => {
                    if *c != color {
                        panic!("bleed");
                    }
                }
                Some(BasinTag::Ridge) | None => {}
            }
        }
        color += 1;
    }

    bmap
}

fn count_basin_sz(map: &HashMap<(isize, isize), BasinTag>) -> HashMap<usize, usize> {
    map.iter().fold(HashMap::new(), |mut acc, (_, t)| match t {
        BasinTag::Basin(c) => {
            acc.entry(*c).and_modify(|c| *c += 1).or_insert(1);
            acc
        }
        BasinTag::Unknown => {
            panic!("bad map");
        }
        BasinTag::Ridge => acc,
    })
}

#[cfg(test)]
mod day9_tests {
    use aoc2021::str_as_vec;

    use super::*;

    #[test]
    fn test_low_check() {
        let test_data = str_as_vec("2222222222\n2222122222\n2222222222\n");
        let map = lines_to_map(&test_data);

        assert!(is_low(&map, 4, 1));
        assert!(!is_low(&map, 0, 0));
    }

    #[test]
    fn test_lines_to_map() {
        let test_data = str_as_vec("2199943210\n3987894921");

        let map = lines_to_map(&test_data);

        assert_eq!(*map.get(&(0, 0)).unwrap(), '2');
        assert_eq!(*map.get(&(0, 1)).unwrap(), '3');
        assert_eq!(*map.get(&(1, 0)).unwrap(), '1');
    }

    #[test]
    fn test_case_pt1() {
        let test_data = str_as_vec(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );

        let map = lines_to_map(&test_data);

        let soln = map
            .iter()
            .map(|(&(x, y), &c)| (c, is_low(&map, x, y)))
            .filter(|(_, l)| *l)
            .fold(0, |acc, (c, _)| acc + c.to_digit(10).unwrap() + 1);

        assert_eq!(soln, 15);
    }

    #[test]
    fn test_case_pt2() {
        let test_data = str_as_vec(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );

        let map = lines_to_map(&test_data);
        println!("{:?}", map);
        let tmap = tag_basin(&map);
        println!("{:?}", tmap);

        let bsz = count_basin_sz(&tmap);
        println!("{:?}", bsz);

        let mut v: Vec<usize> = bsz.values().cloned().collect();
        v.sort_unstable();
        let p: usize = v
            .into_iter()
            .rev()
            .take(3)
            .into_iter()
            .reduce(|acc, v| acc * v)
            .unwrap();
        assert_eq!(p, 1134);
    }
}
