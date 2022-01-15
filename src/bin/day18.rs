use aoc2021::lines_as_vec;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let lines = lines_as_vec("input/day18.txt");
    magnitude(&mut reduce_many(&lines))
}

fn part2() -> u32 {
    let lines = lines_as_vec("input/day18.txt");
    lines
        .iter()
        .flat_map(|x| {
            lines.iter().map(|y| {
                let mut sum = add_nums(&tokenize(x), &tokenize(y));
                reduce_sn(&mut sum);
                magnitude(&mut sum)
            })
        })
        .max()
        .unwrap()
}

#[derive(Debug, Eq, PartialEq)]
struct Item {
    value: u32,
    depth: usize,
}

type Items = Vec<Item>;

fn tokenize(input: &str) -> Items {
    let mut depth = 0;
    input.chars().fold(Vec::new(), |mut acc, c| {
        match c {
            '[' => depth += 1,
            ',' => {}
            ']' => depth -= 1,
            '0'..='9' => acc.push(Item {
                value: c.to_digit(10).unwrap(),
                depth,
            }),
            _ => unreachable!(),
        }
        acc
    })
}

fn add_nums(l: &Items, r: &Items) -> Items {
    l.iter()
        .chain(r.iter())
        .map(|item| Item {
            value: item.value,
            depth: item.depth + 1,
        })
        .collect()
}

fn explode(items: &mut Items) -> bool {
    for i in 0..(items.len()) {
        if items[i].depth >= 5 && items[i].depth == items[i + 1].depth {
            let d = items[i].depth;
            if i > 0 {
                items[i - 1].value += items[i].value;
            }
            if i < items.len() - 2 {
                items[i + 2].value += items[i + 1].value;
            }
            items.remove(i);
            items.remove(i);
            items.insert(
                i,
                Item {
                    value: 0,
                    depth: d - 1,
                },
            );
            return true;
        }
    }
    false
}

fn split(items: &mut Items) -> bool {
    for i in 0..(items.len()) {
        if items[i].value > 9 {
            let Item { value, depth } = items[i];
            items.remove(i);
            items.insert(
                i,
                Item {
                    value: (value as f32 / 2.0).ceil() as u32,
                    depth: depth + 1,
                },
            );
            items.insert(
                i,
                Item {
                    value: (value as f32 / 2.0).floor() as u32,
                    depth: depth + 1,
                },
            );
            return true;
        }
    }
    false
}

fn reduce_sn(items: &mut Items) {
    'outer: loop {
        if explode(items) {
            //print_items(items);
            continue 'outer;
        }

        if split(items) {
            //print_items(items);
            continue 'outer;
        }

        break;
    }
}

fn reduce_many(lines: &[String]) -> Items {
    lines
        .iter()
        .map(|line| tokenize(line))
        .reduce(|acc, right| {
            let mut sum = add_nums(&acc, &right);
            reduce_sn(&mut sum);
            sum
        })
        .unwrap()
}

fn magnitude(items: &mut Items) -> u32 {
    'outer: while items.len() > 1 {
        for i in 0..(items.len()) {
            if items[i].depth == items[i + 1].depth {
                let value = items[i].value * 3 + items[i + 1].value * 2;
                let depth = items[i].depth;
                items.remove(i);
                items.remove(i);
                items.insert(
                    i,
                    Item {
                        value,
                        depth: depth - 1,
                    },
                );
                continue 'outer;
            }
        }
    }
    items[0].value
}

#[cfg(test)]
mod day18_tests {

    use aoc2021::str_as_vec;

    use super::*;

    #[test]
    fn test_split() {
        // [[[[0,7],4],[15,[0,13]]],[1,1]]
        let mut tok = vec![
            Item { value: 0, depth: 4 },
            Item { value: 7, depth: 4 },
            Item { value: 4, depth: 3 },
            Item {
                value: 15,
                depth: 3,
            },
            Item { value: 0, depth: 4 },
            Item {
                value: 13,
                depth: 4,
            },
            Item { value: 1, depth: 2 },
            Item { value: 1, depth: 2 },
        ];
        split(&mut tok);
        split(&mut tok);
        assert_eq!(tokenize("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"), tok);
    }

    #[test]
    fn test_add() {
        let left = tokenize("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = tokenize("[1,1]");
        let sum = add_nums(&left, &right);

        assert_eq!(tokenize("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"), sum);
    }

    #[test]
    fn test_explode() {
        let mut tok = tokenize("[[[[[9,8],1],2],3],4]");
        explode(&mut tok);
        assert_eq!(tokenize("[[[[0,9],2],3],4]"), tok);

        let mut tok = tokenize("[7,[6,[5,[4,[3,2]]]]]");
        explode(&mut tok);
        assert_eq!(tokenize("[7,[6,[5,[7,0]]]]"), tok);

        let mut tok = tokenize("[[6,[5,[4,[3,2]]]],1]");
        explode(&mut tok);
        assert_eq!(tokenize("[[6,[5,[7,0]]],3]"), tok);

        let mut tok = tokenize("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        explode(&mut tok);
        assert_eq!(tokenize("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), tok);
    }

    #[test]
    fn test_tokenize() {
        let s = "[[[[[9,8],1],2],3],4]";
        assert_eq!(
            vec![
                Item { value: 9, depth: 5 },
                Item { value: 8, depth: 5 },
                Item { value: 1, depth: 4 },
                Item { value: 2, depth: 3 },
                Item { value: 3, depth: 2 },
                Item { value: 4, depth: 1 },
            ],
            tokenize(s)
        );
    }

    #[test]
    fn test_sum_reduce() {
        let left = tokenize("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = tokenize("[1,1]");
        let mut sum = add_nums(&left, &right);

        reduce_sn(&mut sum);
        assert_eq!(tokenize("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), sum);
    }

    #[test]
    fn test_mag() {
        assert_eq!(143, magnitude(&mut tokenize("[[1,2],[[3,4],5]]")));

        assert_eq!(
            1384,
            magnitude(&mut tokenize("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"))
        );

        assert_eq!(
            445,
            magnitude(&mut tokenize("[[[[1,1],[2,2]],[3,3]],[4,4]]"))
        );

        assert_eq!(
            791,
            magnitude(&mut tokenize("[[[[3,0],[5,3]],[4,4]],[5,5]]"))
        );
    }

    #[test]
    fn test_case_p1() {
        let test_data = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

        let lines = str_as_vec(test_data);
        let result = reduce_many(&lines);

        assert_eq!(
            tokenize("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            result
        );

        let test_data = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let lines = str_as_vec(test_data);
        let mut toks = reduce_many(&lines);
        assert_eq!(4140, magnitude(&mut toks));
    }
}
