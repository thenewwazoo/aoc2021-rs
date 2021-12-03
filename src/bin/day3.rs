use aoc2021::read_lines_from;

pub fn main() {
    println!("Solution: {}", part1());
}

fn part1() -> u64 {
    let lines = &read_lines_from("input/day3.txt")
        .unwrap() // die if we can't read the file
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();
    let mut out = setup_acc(&lines[0]);
    lines.iter().for_each(|l| merge_bitstr(&mut out, l));
    let mf = most_fewest(&out);
    let c = coll_most_fewest(&mf);
    let (gamma, epsilon) = (to_u64(&c.0), to_u64(&c.1));
    gamma * epsilon
}

/// turn a slice of bits into a number
fn to_u64(slice: &[bool]) -> u64 {
    slice
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &b)| acc + ((b as u64) << i))
}

/// Create a Vec of the same length as the str, full of empty Vecs
fn setup_acc(ex: &str) -> Vec<Vec<bool>> {
    let mut v = vec![];
    (0..(ex.len())).for_each(|_| v.push(Vec::new()));
    v
}

/// add character-bits (1s and 0s) to the provided acc
fn merge_bitstr(acc: &mut [Vec<bool>], bitstr: &str) {
    bitstr
        .chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(()),
        })
        .collect::<Result<Vec<bool>, ()>>()
        .unwrap() // die if we get anything but a 1 or 0
        .into_iter()
        .zip(acc.iter_mut())
        .for_each(|(c, a)| a.push(c))
}

/// Given a Vec, return a tuple with the more- and less- common value in the vec, respectively. Do
/// this for every Vec in the list
fn most_fewest(data: &[Vec<bool>]) -> Vec<(bool, bool)> {
    data.iter()
        .map(|v| {
            v.iter().fold((0, 0), |acc, v| match v {
                true => (acc.0 + 1, acc.1),
                false => (acc.0, acc.1 + 1),
            })
        })
        .map(|(true_ct, false_ct)| {
            if true_ct == false_ct {
                panic!("equal");
            }

            if true_ct > false_ct {
                (true, false)
            } else {
                (false, true)
            }
        })
        .collect()
}

/// Turn a collection of tuples into a tuple of collections
fn coll_most_fewest(data: &[(bool, bool)]) -> (Vec<bool>, Vec<bool>) {
    data.iter()
        .fold((Vec::new(), Vec::new()), |(mut m, mut f), v| {
            m.push(v.0);
            f.push(v.1);
            (m, f)
        })
}

#[cfg(test)]
mod day3_tests {

    use super::*;

    #[test]
    fn test_case() {
        let mut out = setup_acc("01234");
        let test_data = "00100
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
        test_data.lines().for_each(|l| merge_bitstr(&mut out, l));
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

        assert_eq!(most_fewest(&d), vec![(false, true), (true, false)],);
    }

    #[test]
    fn test_merge_bitstr() {
        let mut testv = vec![vec![], vec![], vec![], vec![], vec![]];
        let res = vec![
            vec![false],
            vec![false],
            vec![true],
            vec![false],
            vec![false],
        ];

        merge_bitstr(&mut testv, "00100");

        assert_eq!(testv, res);

        let res = vec![
            vec![false, true],
            vec![false, true],
            vec![true, true],
            vec![false, true],
            vec![false, false],
        ];

        merge_bitstr(&mut testv, "11110");

        assert_eq!(testv, res);
    }

    #[test]
    fn test_setup_acc() {
        assert_eq!(setup_acc("01234").len(), 5,);
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
        let d = vec![(true, false), (false, true), (true, false)];
        assert_eq!(
            coll_most_fewest(&d),
            (vec![true, false, true], vec![false, true, false])
        );
    }
}
