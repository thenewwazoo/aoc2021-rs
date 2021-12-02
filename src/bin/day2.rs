use aoc2021::read_lines_from;
use aoc2021::sub::{Nav, NavParseError, Sub, SubError};

fn main() {
    println!("Solution: {}", part1());
}

fn part1() -> u64 {
    let mut sub: Sub = Sub::default();

    let lines = &read_lines_from("input/day2.txt")
        .unwrap() // die if we can't read the file
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();

    let navs = lines_to_navs(lines).unwrap(); // die if there's a bad line

    navigate_sub(&mut sub, navs).unwrap();

    sub.dist * sub.depth
}

fn lines_to_navs(lines: &[String]) -> Result<Vec<Nav>, NavParseError> {
    lines
        .iter()
        .map(|l| Nav::try_from(l.as_str()))
        .collect::<Result<Vec<Nav>, NavParseError>>()
}

fn navigate_sub(sub: &mut Sub, moves: Vec<Nav>) -> Result<(), SubError> {
    moves
        .into_iter()
        .try_for_each(|nav| sub.try_move(nav))
}

#[cfg(test)]
mod day2_tests {

    use aoc2021::sub::Nav;
    use super::*;

    const TEST_DATA_STR: &str =
"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_nav_gen() {
        let test_data: Vec<String> = TEST_DATA_STR
            .lines()
            .map(|l| l.to_string())
            .collect();

        assert_eq!(
            Ok(vec![Nav::Fore(5), Nav::Down(5), Nav::Fore(8), Nav::Up(3), Nav::Down(8), Nav::Fore(2)]),
            lines_to_navs(&test_data)
        )
    }

    #[test]
    fn test_move_sub() {
        let test_data: Vec<Nav> = lines_to_navs(
            TEST_DATA_STR
                .lines()
                .map(|l| l.to_string())
                .collect::<Vec<String>>()
                .as_slice()
        ).unwrap();

        let mut s = Sub::default();
        let nav_result = navigate_sub(&mut s, test_data);

        assert_eq!(nav_result, Ok(()));
        assert_eq!(s, Sub{dist: 15, depth: 60, aim: 10});
    }

}
