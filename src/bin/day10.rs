use aoc2021::lines_as_vec;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u64 {
    let lines = lines_as_vec("input/day10.txt");

    lines
        .iter()
        .map(find_corruption)
        .map(|c| match c {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            Some(_) => panic!("bad char"),
            None => 0,
        })
        .sum()
}

fn part2() -> u64 {
    let lines = lines_as_vec("input/day10.txt");

    let mut scores: Vec<u64> = lines
        .iter()
        .filter(|l| find_corruption(l).is_none())
        .map(complete_line)
        .map(|cont| score_cont(&cont))
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn find_corruption(line: &String) -> Option<char> {
    let mut acc = Vec::new();
    line.chars()
        .map(|c| check_last(&mut acc, c))
        .collect::<Result<Vec<()>, char>>()
        .err()
}

fn check_last(acc: &mut Vec<char>, c: char) -> Result<(), char> {
    match c {
        '(' | '[' | '{' | '<' => {
            acc.push(c);
            Ok(())
        }
        ')' => match acc.pop() {
            Some('(') => Ok(()),
            _ => Err(c),
        },
        '}' => match acc.pop() {
            Some('{') => Ok(()),
            _ => Err(c),
        },
        ']' => match acc.pop() {
            Some('[') => Ok(()),
            _ => Err(c),
        },
        '>' => match acc.pop() {
            Some('<') => Ok(()),
            _ => Err(c),
        },
        _ => panic!("bad check char"),
    }
}

fn score_cont(cont: &str) -> u64 {
    cont.chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("bad sc"),
        })
        .fold(0, |acc, s| acc * 5 + s)
}

fn complete_line(line: &String) -> String {
    let mut stack = Vec::new();
    line.chars().for_each(|c| match c {
        '(' | '[' | '{' | '<' => {
            stack.push(c);
        }
        ')' | ']' | '}' | '>' => {
            stack.pop().unwrap();
        }
        _ => panic!("bad char"),
    });
    stack
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            '<' => '>',
            _ => panic!("bad char"),
        })
        .collect()
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn test_score_cont() {
        assert_eq!(294, score_cont("])}>"));
        assert_eq!(288957, score_cont("}}]])})]"));
        assert_eq!(5566, score_cont(")}>]})"));
        assert_eq!(1480781, score_cont("}}>}>))))"));
        assert_eq!(995444, score_cont("]]}}]}]}>"));
    }

    #[test]
    fn test_complete_line() {
        assert_eq!(
            "}}]])})]",
            complete_line(&String::from("[({(<(())[]>[[{[]{<()<>>"))
        );
        assert_eq!(
            ")}>]})",
            complete_line(&String::from("[(()[<>])]({[<{<<[]>>("))
        );
        assert_eq!(
            "}}>}>))))",
            complete_line(&String::from("(((({<>}<{<{<>}{[]{[]{}"))
        );
        assert_eq!(
            "]]}}]}]}>",
            complete_line(&String::from("{<[[]]>}<{[{[{[]{()[[[]"))
        );
        assert_eq!(
            "])}>",
            complete_line(&String::from("<{([{{}}[<[[[<>{}]]]>[]]"))
        );
    }

    #[test]
    fn test_find_corruption() {
        assert_eq!(None, find_corruption(&String::from("({{")));
        assert_eq!(None, find_corruption(&String::from("([{<>}])")));
        assert_eq!(Some(']'), find_corruption(&String::from("(()]")));
    }

    #[test]
    fn test_check_last() {
        let mut acc = vec!['('];
        assert_eq!(Ok(()), check_last(&mut acc, ')'));

        let mut acc = vec!['('];
        assert_eq!(Err(']'), check_last(&mut acc, ']'));
    }
}
