use std::collections::HashMap;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> usize {
    let (p1, p2, die) = play_part1(9, 6);
    (die.0 - 1) * if p1.score >= 1000 { p2.score } else { p1.score }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct State {
    pos: Vec<u8>,
    scores: Vec<u8>,
    player: u8,
    count: u64,
}

fn part2() -> u64 {
    // Precompute counts of dice rolls.
    let rolls = {
        let mut histo = HashMap::new();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    *histo.entry(i + j + k).or_default() += 1;
                }
            }
        }
        let mut rolls = histo
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(usize, usize)>>();
        rolls.sort();
        rolls
    };

    // I expressed this as a game-tree search problem.
    //
    // We express the number of universes on the same "path" via the 'count' parameter.  Without
    // this, the number grows too quickly.  E.g., when you roll 3 dice, there is one universe where
    // you get 1+1+1, but there are 3 universes where you get 1+1+2.
    let state = State {
        pos: vec![9, 6],
        scores: vec![0, 0],
        player: 0,
        count: 1,
    };

    // Since we need to evaluate the entire space, depth-first search is most efficient.
    let mut queue = Vec::new();
    queue.push(state);

    let mut wins = vec![0, 0];

    while queue.len() > 0 {
        let next = queue.pop().unwrap();

        // Enumerate eligible moves.
        for (roll, newcounts) in rolls.iter() {
            let mut newstate = next.clone();
            newstate.pos[next.player as usize] =
                ((newstate.pos[next.player as usize] - 1 + *roll as u8) % 10) + 1;
            newstate.scores[next.player as usize] += newstate.pos[next.player as usize];
            newstate.count *= *newcounts as u64;

            // Count win conditions.
            if newstate.scores[next.player as usize] >= 21 {
                wins[next.player as usize] += newstate.count;
                continue;
            }

            newstate.player = (newstate.player + 1) % 2;
            queue.push(newstate);
        }
    }
    *wins.iter().max().unwrap()
}

const BOARD_SZ: usize = 10;

struct Pawn {
    space: usize,
    score: usize,
}

impl Pawn {
    fn new(space: usize) -> Self {
        Pawn { space, score: 0 }
    }

    fn move_pawn(&mut self, dist: usize) {
        self.space = ((self.space + dist - 1) % BOARD_SZ) + 1;
        self.score += self.space;
    }
}

struct DetDie(usize);

impl DetDie {
    fn new() -> Self {
        Self(1)
    }

    fn next(&mut self, count: usize) -> usize {
        let r = ((self.0)..(self.0 + count)).sum::<usize>();
        self.0 += count;
        r
    }
}

fn play_part1(p1_start: usize, p2_start: usize) -> (Pawn, Pawn, DetDie) {
    let mut die = DetDie::new();
    let mut p1 = Pawn::new(p1_start);
    let mut p2 = Pawn::new(p2_start);

    loop {
        p1.move_pawn(die.next(3));
        if p1.score >= 1000 {
            break;
        }

        p2.move_pawn(die.next(3));
        if p2.score >= 1000 {
            break;
        }
    }

    (p1, p2, die)
}

#[cfg(test)]
mod day21_tests {

    use super::*;

    #[test]
    fn test_die() {
        let mut d = DetDie::new();

        assert_eq!(d.next(1), 1);
        assert_eq!(d.0, 2);
        assert_eq!(d.next(2), 5);
        assert_eq!(d.0, 4);
        assert_eq!(d.next(3), 15);
        assert_eq!(d.0, 7);
    }

    #[test]
    fn test_move() {
        let mut p1 = Pawn::new(4);
        let mut p2 = Pawn::new(8);

        p1.move_pawn(1 + 2 + 3);
        assert_eq!(p1.space, 10);
        assert_eq!(p1.score, 10);

        p2.move_pawn(4 + 5 + 6);
        assert_eq!(p2.space, 3);
        assert_eq!(p2.score, 3);

        p1.move_pawn(7 + 8 + 9);
        assert_eq!(p1.space, 4);
        assert_eq!(p1.score, 14);

        p2.move_pawn(10 + 11 + 12);
        assert_eq!(p2.space, 6);
        assert_eq!(p2.score, 9);

        p1.move_pawn(13 + 14 + 15);
        assert_eq!(p1.space, 6);
        assert_eq!(p1.score, 20);

        p2.move_pawn(16 + 17 + 18);
        assert_eq!(p2.space, 7);
        assert_eq!(p2.score, 16);
    }

    #[test]
    fn test_case() {
        let (p1, p2, die) = play_part1(4, 8);
        assert_eq!(die.0 - 1, 993);
        assert_eq!(p2.score, 745);
    }
}
