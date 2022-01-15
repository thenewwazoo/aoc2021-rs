use aoc2021::lines_as_vec;

use std::collections::VecDeque;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let lines = lines_as_vec("input/day20.txt");
    let (e, mut i) = parse(&lines);

    expand(&mut i);
    let mut i = enhance(i, &e);

    expand(&mut i);
    let i = enhance(i, &e);

    i.iter().flatten().filter(|&b| *b == b'#').count()
}

fn parse(lines: &[String]) -> (Vec<u8>, VecDeque<Vec<u8>>) {
    let enhancer: Vec<u8> = lines[0].clone().as_bytes().into_iter().cloned().collect();

    let img: VecDeque<Vec<u8>> = lines[2..]
        .into_iter()
        .map(|l| l.as_bytes().into_iter().cloned().collect())
        .collect();

    (enhancer, img)
}

fn expand(s: &mut VecDeque<Vec<u8>>) {
    // every iteration, we increase the height and width by no more than 4
    let width = s[0].len() + 4;
    s.iter_mut().for_each(|l| {
        l.insert(0, b'.');
        l.insert(0, b'.');
        l.push(b'.');
        l.push(b'.');
    });
    s.push_front(Vec::from_iter(vec![b'.'; width]));
    s.push_front(Vec::from_iter(vec![b'.'; width]));
    s.push_back(Vec::from_iter(vec![b'.'; width]));
    s.push_back(Vec::from_iter(vec![b'.'; width]));
}

fn str_to_idx(s: &[u8]) -> usize {
    s.iter().rev().enumerate().fold(
        0,
        |acc, (i, &c)| if c == b'#' { acc | (1 << i) } else { acc },
    )
}

fn enhance(s: VecDeque<Vec<u8>>, enhancer: &[u8]) -> VecDeque<Vec<u8>> {
    let mut out = s.clone();
    for y in 1..(s.len() - 1) {
        for x in 1..(s[0].len() - 1) {
            let bitstr: Vec<u8> = [
                Vec::from(&s[y - 1][(x - 1)..=(x + 1)]),
                Vec::from(&s[y][(x - 1)..=(x + 1)]),
                Vec::from(&s[y + 1][(x - 1)..=(x + 1)]),
            ]
            .into_iter()
            .flatten()
            .collect();
            let idx = str_to_idx(&bitstr);
            out[y][x] = enhancer[idx];
        }
    }
    out
}

/*

#[cfg(test)]
mod day20_tests {

    use aoc2021::str_as_vec;

    use super::*;

    #[test]
    fn test_enhance() {
        let mut v = VecDeque::from_iter([
            Vec::from_iter(String::from("...").into_bytes()),
            Vec::from_iter(String::from("...").into_bytes()),
            Vec::from_iter(String::from("#.#").into_bytes()),
        ]);
        let e = Vec::from_iter(String::from("....#").into_bytes());
        let v = enhance(v, &e);
        assert_eq!(
            VecDeque::from_iter([
                Vec::from_iter(String::from("...").into_bytes()),
                Vec::from_iter(String::from("...").into_bytes()),
                Vec::from_iter(String::from("#.#").into_bytes()),
            ]),
            v
        );
    }

    #[test]
    fn test_expand() {
        let mut v = VecDeque::from_iter([
            Vec::from_iter(String::from("...").into_bytes()),
            Vec::from_iter(String::from("...").into_bytes()),
            Vec::from_iter(String::from("...").into_bytes()),
        ]);
        expand(&mut v);
        assert_eq!(
            VecDeque::from_iter([
                Vec::from_iter(String::from(".......").into_bytes()),
                Vec::from_iter(String::from(".......").into_bytes()),
                Vec::from_iter(String::from(".......").into_bytes()),
                Vec::from_iter(String::from(".......").into_bytes()),
                Vec::from_iter(String::from(".......").into_bytes()),
                Vec::from_iter(String::from(".......").into_bytes()),
                Vec::from_iter(String::from(".......").into_bytes()),
            ]),
            v
        );
    }

    #[test]
    fn test_str_to_idx() {
        assert_eq!(2, str_to_idx(".......#.".as_bytes()));
        assert_eq!(2usize.pow(9) - 1, str_to_idx("#########".as_bytes()));
    }

    #[test]
    fn test_case() {
        let test_data = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        let (e, mut i) = parse(&str_as_vec(test_data));
        // print_i(&i);

        expand(&mut i);
        let mut i = enhance(i, &e);
        // print_i(&i);

        expand(&mut i);
        let mut i = enhance(i, &e);
        // print_i(&i);
        println!("{}", i.iter().flatten().filter(|&b| *b == b'#').count());

        assert!(false);
    }
}

/*
fn print_i(i: &VecDeque<Vec<u8>>) {
    for l in i {
        println!("{}", String::from_utf8(l.to_vec()).unwrap());
    }
}
*/

*/
