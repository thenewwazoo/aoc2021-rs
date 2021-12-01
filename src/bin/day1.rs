use aoc2021::read_lines_from;

pub fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

pub fn part1() -> usize {
    find_increases(
        &read_lines_from("input/day1.txt")
        .unwrap()
        .map(|l| l.unwrap().parse::<u64>().expect("not a number"))
        .collect::<Vec<u64>>()
    )
}

pub fn part2() -> usize {
    find_increases(
        &group_lines(
            &read_lines_from("input/day1.txt")
                .unwrap()
                .map(|l| l.unwrap().parse::<u64>().expect("not a number"))
                .collect::<Vec<u64>>()
        )
    )
}

fn find_increases(m: &[u64]) -> usize {
    m
        .windows(2)
        .map(|w| match w {
            &[] => false,
            &[_] => false,
            [a, b, ..] => b > a,
        })
    .filter(|&a| a)
        .count()
}

fn group_lines(m: &[u64]) -> Vec<u64> {
    m
        .windows(3)
        .map(|w| w.iter().sum())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_DATA_STR: &str =
        &"199\n\
          200\n\
          208\n\
          210\n\
          200\n\
          207\n\
          240\n\
          269\n\
          260\n\
          263";

    #[test]
    fn test_find_increases() {
        let test_data = TEST_DATA_STR
            .lines()
            .map(|l| l.parse::<u64>().expect("not a number"))
            .collect::<Vec<u64>>();
        assert_eq!(find_increases(&test_data), 7);
    }


    #[test]
    fn test_find_increases_multi() {
        let test_data = TEST_DATA_STR
            .lines()
            .map(|l| l.parse::<u64>().expect("not a number"))
            .collect::<Vec<u64>>();
        assert_eq!(
            find_increases(&group_lines(&test_data)),
            5
        );
    }
}
