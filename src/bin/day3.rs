use aoc2021::read_lines_from;

pub fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u64 {
    let lines = &read_lines_from("input/day3.txt")
        .unwrap() // die if we can't read the file
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();
    let bitstrs: Vec<Vec<bool>> = lines.iter().map(|l| str_to_bitvec(l)).collect();
    let out = transpose(&bitstrs);
    let mf = most_fewest(&out);
    let c = coll_most_fewest(&mf);
    let (gamma, epsilon) = (to_u64(&c.0), to_u64(&c.1));
    gamma * epsilon
}

fn part2() -> u64 {
    let lines = &read_lines_from("input/day3.txt")
        .unwrap() // die if we can't read the file
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();
    let bitstrs: Vec<Vec<bool>> = lines.iter().map(|l| str_to_bitvec(l)).collect();
    let ox = filter_ox(&bitstrs);
    let co2 = filter_co2(&bitstrs);
    to_u64(&ox) * to_u64(&co2)
}

/// turn a slice of bits into a number
fn to_u64(slice: &[bool]) -> u64 {
    slice
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &b)| acc + ((b as u64) << i))
}

fn str_to_bitvec(bitstr: &str) -> Vec<bool> {
    bitstr
        .chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(()),
        })
        .collect::<Result<Vec<bool>, ()>>()
        .unwrap() // die if we get anything but a 1 or a 0
}

fn transpose(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut out: Vec<Vec<bool>> = (0..(data[0].len())).map(|_| Vec::new()).collect();
    data.iter()
        .for_each(|l| l.iter().enumerate().for_each(|(i, &d)| out[i].push(d)));
    out
}

/// Given a Vec, return a tuple with the more- and less- common value in the vec, respectively. Do
/// this for every Vec in the list
fn most_fewest(data: &[Vec<bool>]) -> Vec<Option<(bool, bool)>> {
    data.iter()
        .map(|v| {
            v.iter().fold((0, 0), |acc, v| match v {
                true => (acc.0 + 1, acc.1),
                false => (acc.0, acc.1 + 1),
            })
        })
        .map(|(true_ct, false_ct)| {
            if true_ct == false_ct {
                return None;
            }

            if true_ct > false_ct {
                Some((true, false))
            } else {
                Some((false, true))
            }
        })
        .collect()
}

/// Turn a collection of tuples into a tuple of collections
fn coll_most_fewest(data: &[Option<(bool, bool)>]) -> (Vec<bool>, Vec<bool>) {
    data.iter()
        .map(|v| v.unwrap())
        .fold((Vec::new(), Vec::new()), |(mut m, mut f), v| {
            m.push(v.0);
            f.push(v.1);
            (m, f)
        })
}

fn filter_pop(data: Vec<Vec<bool>>, idx: usize, pop: bool) -> Vec<Vec<bool>> {
    let t = transpose(&data);
    let mf = most_fewest(&t);
    data.clone()
        .into_iter()
        .filter(|v| {
            v[idx]
                == match mf[idx] {
                    Some(t) => {
                        if pop {
                            t.0
                        } else {
                            t.1
                        }
                    }
                    None => pop,
                }
        })
        .collect()
}

fn filter_ox(bitstrs: &[Vec<bool>]) -> Vec<bool> {
    let mut data = bitstrs.to_vec();
    let mut idx = 0;
    while data.len() > 1 {
        data = filter_pop(data, idx, true);
        idx += 1;
    }
    data[0].clone()
}

fn filter_co2(bitstrs: &[Vec<bool>]) -> Vec<bool> {
    let mut data = bitstrs.to_vec();
    let mut idx = 0;
    while data.len() > 1 {
        data = filter_pop(data, idx, false);
        idx += 1;
    }
    data[0].clone()
}

#[cfg(test)]
mod day3_tests {

    use super::*;

    const TEST_DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_case() {
        let bitstrs: Vec<Vec<bool>> = TEST_DATA.lines().map(|l| str_to_bitvec(l)).collect();
        let out = transpose(&bitstrs);
        let mf = most_fewest(&out);
        let c = coll_most_fewest(&mf);
        assert_eq!(c.0, vec![true, false, true, true, false]);
        assert_eq!(c.1, vec![false, true, false, false, true]);
        let (gamma, epsilon) = (to_u64(&c.0), to_u64(&c.1));
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);

        let power_consumption = gamma * epsilon;
        assert_eq!(power_consumption, 198);
    }

    #[test]
    fn test_most_fewest() {
        let d = vec![vec![false, false, true], vec![true, true, false]];

        assert_eq!(
            most_fewest(&d)
                .into_iter()
                .map(Option::unwrap)
                .collect::<Vec<(bool, bool)>>(),
            vec![(false, true), (true, false)],
        );
    }

    #[test]
    fn test_to_u64() {
        assert_eq!(to_u64(&vec![true]), 0b1);
        assert_eq!(to_u64(&vec![true, false]), 0b10);
        assert_eq!(to_u64(&vec![true, true]), 0b11);
        assert_eq!(to_u64(&vec![true, false, false]), 0b100);
        assert_eq!(to_u64(&vec![true, true, true]), 0b111);
        assert_eq!(to_u64(&vec![true, false, true, true, false]), 0b10110);
    }

    #[test]
    fn test_coll_most_fewest() {
        let d = vec![
            Some((true, false)),
            Some((false, true)),
            Some((true, false)),
        ];
        assert_eq!(
            coll_most_fewest(&d),
            (vec![true, false, true], vec![false, true, false])
        );
    }

    #[test]
    fn test_transpose() {
        let res = vec![
            vec![false, true],
            vec![false, true],
            vec![true, true],
            vec![false, true],
            vec![false, false],
        ];

        assert_eq!(
            transpose(&res),
            vec![
                vec![false, false, true, false, false],
                vec![true, true, true, true, false],
            ]
        );
    }

    #[test]
    fn test_filter_pop() {
        let res = vec![vec![false, true], vec![false, true], vec![true, true]];

        assert_eq!(
            filter_pop(res.clone(), 0, true),
            vec![vec![false, true], vec![false, true]]
        );

        assert_eq!(filter_pop(res, 0, false), vec![vec![true, true]],);
    }

    #[test]
    fn test_filter_ox() {
        let bitstrs: Vec<Vec<bool>> = TEST_DATA.lines().map(|l| str_to_bitvec(l)).collect();
        assert_eq!(filter_ox(&bitstrs), vec![true, false, true, true, true]);
    }

    #[test]
    fn test_filter_co2() {
        let bitstrs: Vec<Vec<bool>> = TEST_DATA.lines().map(|l| str_to_bitvec(l)).collect();
        assert_eq!(filter_co2(&bitstrs), vec![false, true, false, true, false]);
    }
}
