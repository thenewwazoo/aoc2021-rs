use aoc2021::lines_as_vec;

use std::collections::{HashMap, HashSet};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let data = lines_as_vec("input/day8.txt");

    data.iter()
        .flat_map(|l| {
            l.split(' ').map(|w| w.to_string()).collect::<Vec<String>>()[11..=14].to_vec()
        })
        .filter(|w| matches!(w.len(), 2 | 3 | 4 | 7))
        .count()
}

#[derive(Debug, Clone)]
struct Wiring(HashMap<char, HashSet<char>>);

impl Wiring {
    fn whittle(&mut self, idx: char, upd: &HashSet<char>) {
        self.0
            .insert(idx, self.0[&idx].intersection(upd).cloned().collect());
    }

    fn light(&self, pins: &str) -> u8 {
        let t = self.finalize();
        let mut output: Vec<char> = pins.chars().map(|p| t[&p]).collect();
        output.sort_unstable();
        match output.as_slice() {
            ['A', 'B', 'C', 'E', 'F', 'G'] => 0,
            ['C', 'F'] => 1,
            ['A', 'C', 'D', 'E', 'G'] => 2,
            ['A', 'C', 'D', 'F', 'G'] => 3,
            ['B', 'C', 'D', 'F'] => 4,
            ['A', 'B', 'D', 'F', 'G'] => 5,
            ['A', 'B', 'D', 'E', 'F', 'G'] => 6,
            ['A', 'C', 'F'] => 7,
            ['A', 'B', 'C', 'D', 'E', 'F', 'G'] => 8,
            ['A', 'B', 'C', 'D', 'F', 'G'] => 9,
            _ => panic!("bad pin: {:?}", output),
        }
    }

    fn finalize(&self) -> HashMap<char, char> {
        self.0
            .iter()
            .map(|(output, input)| (*(input.iter().next().unwrap()), *output))
            .collect()
    }
}

impl Default for Wiring {
    fn default() -> Self {
        Self(HashMap::from([
            ('A', HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
            ('B', HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
            ('C', HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
            ('D', HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
            ('E', HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
            ('F', HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
            ('G', HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
        ]))
    }
}

fn part2() -> u32 {
    let data = lines_as_vec("input/day8.txt");

    data.iter()
        .map(|line| {
            let words: Vec<String> = line.split(' ').map(|w| w.to_string()).collect();

            let solution = solve_line(&words[0..=10]);

            words[11..].iter().rev().enumerate().fold(0, |acc, (i, w)| {
                acc + solution.light(w) as u32 * 10u32.pow(i as u32)
            })
        })
        .sum()
}

fn solve_line(data: &[String]) -> Wiring {
    let mut solution = Wiring::default();

    // get the number 1
    let one = data
        .iter()
        .find(|d| d.len() == 2)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    // we can find signal A right away by comparing a 1 with a 7
    solution.whittle(
        'A',
        &HashSet::from_iter(
            data.iter()
                .find(|d| d.len() == 3)
                .unwrap()
                .chars()
                .filter(|c| !one.as_slice().contains(c)),
        ),
    );

    // C or F is what's shared between 1 and 7
    let c_or_f: HashSet<char> = one
        .iter()
        .copied()
        .filter(|c| !solution.0[&'A'].contains(c))
        .collect();
    solution.whittle('C', &c_or_f);
    solution.whittle('F', &c_or_f);

    // B and D are what we know aren't C or F in the digit 4
    let b_or_d: HashSet<char> = HashSet::from_iter(
        data.iter()
            .find(|d| d.len() == 4)
            .unwrap()
            .chars()
            .filter(|s| !solution.0[&'C'].contains(s) && !solution.0[&'F'].contains(s)),
    );
    solution.whittle('B', &b_or_d);
    solution.whittle('D', &b_or_d);

    // 0, 6, and 9 all share six signals
    let mut zero_six_or_nine: Vec<String> = data.iter().filter(|d| d.len() == 6).cloned().collect();
    zero_six_or_nine.dedup();

    // 0, 6, and 9 overlap in A, B, F, G, but we already know what A is, so we can skip that
    let b_f_g: HashSet<char> = zero_six_or_nine
        .iter()
        .map(|n: &String| HashSet::from_iter(n.chars())) // turn each String into a HashSet of chars
        .reduce(|acc: HashSet<char>, n: HashSet<char>| {
            acc.intersection(&n).cloned().collect::<HashSet<_>>() // store only the intersection of all of them
        })
        .unwrap()
        .into_iter()
        .filter(|c| !solution.0[&'A'].contains(c))
        .collect();
    solution.whittle('B', &b_f_g);
    solution.whittle('F', &b_f_g);
    solution.whittle('G', &b_f_g);

    // 2, 3, and 5 all share five signals
    let mut two_three_or_five: Vec<String> =
        data.iter().filter(|d| d.len() == 5).cloned().collect();
    two_three_or_five.dedup();

    // 2, 3, and 5 overlap in A, D, G. as before, we know what A is already
    let d_g: HashSet<char> = two_three_or_five
        .iter()
        .map(|n: &String| HashSet::from_iter(n.chars())) // turn each String into a HashSet of chars
        .reduce(|acc: HashSet<char>, n: HashSet<char>| {
            acc.intersection(&n).cloned().collect::<HashSet<_>>() // store only the intersection of all of them
        })
        .unwrap()
        .into_iter()
        .filter(|c| !solution.0[&'A'].contains(c))
        .collect();

    solution.whittle('D', &d_g);
    solution.whittle('G', &d_g);

    // that gives us all but C, which cannot be F
    solution.0.insert(
        'C',
        solution.0[&'C']
            .difference(&solution.0[&'F'])
            .cloned()
            .collect(),
    );

    // and E is not in anything else
    let everything_else = ['A', 'B', 'C', 'D', 'F', 'G']
        .iter()
        .fold(HashSet::new(), |acc, idx| &acc | &solution.0[idx]);

    solution.0.insert(
        'E',
        solution.0[&'E']
            .difference(&everything_else)
            .cloned()
            .collect(),
    );

    solution
}

// hah no tests today
