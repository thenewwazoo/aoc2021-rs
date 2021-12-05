use aoc2021::read_lines_from;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u64 {
    let lines = &mut read_lines_from("input/day4.txt")
        .unwrap() // die if we can't read the file
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();

    let first_line = &lines.drain(0..1).collect::<Vec<String>>()[0];
    let moves: Vec<&str> = first_line.split(',').collect();

    let mut boards = load_boards(lines);

    let winner = find_winner(&moves, &mut boards);
    let tot: u64 = winner
        .1
        .tiles
        .iter()
        .filter(|t| !t.marked)
        .map(|t| t.value.parse::<u64>().unwrap())
        .sum();
    winner.0.parse::<u64>().unwrap() * tot
}

fn part2() -> u64 {
    let lines = &mut read_lines_from("input/day4.txt")
        .unwrap() // die if we can't read the file
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();

    let first_line = &lines.drain(0..1).collect::<Vec<String>>()[0];
    let moves: Vec<&str> = first_line.split(',').collect();

    let mut boards = load_boards(lines);

    let winner = find_loser(&moves, &mut boards);
    let tot: u64 = winner
        .1
        .tiles
        .iter()
        .filter(|t| !t.marked)
        .map(|t| t.value.parse::<u64>().unwrap())
        .sum();
    winner.0.parse::<u64>().unwrap() * tot
}

fn find_loser<'a>(moves: &[&'a str], boards: &'a mut [Board]) -> (&'a str, Board<'a>) {
    let mut loser = None;
    let mut remain: Vec<usize> = (0..boards.len()).collect();
    'outer: for num in moves {
        let (mut w, t): (Vec<usize>, Vec<usize>) = remain
            .iter()
            .partition(|&&i| boards[i].play(num) == Some(Outcome::Bingo));
        if t.is_empty() {
            loser = Some((*num, boards[w.pop().unwrap()].clone()));
            break 'outer;
        }
        remain = t;
    }
    loser.unwrap()
}

fn find_winner<'a>(moves: &[&'a str], boards: &'a mut [Board]) -> (&'a str, Board<'a>) {
    let mut winner = None;
    'outer: for num in moves {
        for board in boards.iter_mut() {
            if board.play(num) == Some(Outcome::Bingo) {
                winner = Some((*num, board.clone()));
                break 'outer;
            }
        }
    }
    winner.unwrap()
}

fn load_boards(lines: &[String]) -> Vec<Board> {
    lines
        .chunks(6) // 5 board lines, one (maybe) gap line
        .map(make_board)
        .collect()
}

fn make_board(lines: &[String]) -> Board {
    Board {
        tiles: lines
            .iter()
            .flat_map(|line| line.split_whitespace())
            .map(|v| Tile {
                value: v,
                marked: false,
            })
            .collect(),
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Bingo,
    Nothing,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Board<'t> {
    tiles: Vec<Tile<'t>>,
}

impl<'a> Board<'a> {
    pub fn play(&mut self, value: &str) -> Option<Outcome> {
        let mut exists = false;
        for tile in self.tiles.iter_mut() {
            if tile.value == value {
                tile.marked = true;
                exists = true;
                break;
            }
        }
        if !exists {
            return None;
        }

        if self.check_vert(0)
            || self.check_vert(1)
            || self.check_vert(2)
            || self.check_vert(3)
            || self.check_vert(4)
            || self.check_horiz(0)
            || self.check_horiz(5)
            || self.check_horiz(10)
            || self.check_horiz(15)
            || self.check_horiz(20)
        {
            Some(Outcome::Bingo)
        } else {
            Some(Outcome::Nothing)
        }
    }

    fn check_vert(&'a self, i: usize) -> bool {
        [i, i + 5, i + 10, i + 15, i + 20]
            .iter()
            .map(|&i| self.tiles[i].marked)
            .all(|m| m)
    }

    fn check_horiz(&'a self, i: usize) -> bool {
        (i..i + 5).map(|i| self.tiles[i].marked).all(|m| m)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Tile<'v> {
    value: &'v str,
    marked: bool,
}

#[cfg(test)]
mod day4_tests {

    use super::*;

    #[test]
    fn test_read_board() {
        let test_data = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

";
        let res = Board {
            tiles: vec![
                Tile {
                    value: "22",
                    marked: false,
                },
                Tile {
                    value: "13",
                    marked: false,
                },
                Tile {
                    value: "17",
                    marked: false,
                },
                Tile {
                    value: "11",
                    marked: false,
                },
                Tile {
                    value: "0",
                    marked: false,
                },
                Tile {
                    value: "8",
                    marked: false,
                },
                Tile {
                    value: "2",
                    marked: false,
                },
                Tile {
                    value: "23",
                    marked: false,
                },
                Tile {
                    value: "4",
                    marked: false,
                },
                Tile {
                    value: "24",
                    marked: false,
                },
                Tile {
                    value: "21",
                    marked: false,
                },
                Tile {
                    value: "9",
                    marked: false,
                },
                Tile {
                    value: "14",
                    marked: false,
                },
                Tile {
                    value: "16",
                    marked: false,
                },
                Tile {
                    value: "7",
                    marked: false,
                },
                Tile {
                    value: "6",
                    marked: false,
                },
                Tile {
                    value: "10",
                    marked: false,
                },
                Tile {
                    value: "3",
                    marked: false,
                },
                Tile {
                    value: "18",
                    marked: false,
                },
                Tile {
                    value: "5",
                    marked: false,
                },
                Tile {
                    value: "1",
                    marked: false,
                },
                Tile {
                    value: "12",
                    marked: false,
                },
                Tile {
                    value: "20",
                    marked: false,
                },
                Tile {
                    value: "15",
                    marked: false,
                },
                Tile {
                    value: "19",
                    marked: false,
                },
            ],
        };

        let d = test_data
            .lines()
            .map(str::to_string)
            .collect::<Vec<String>>();
        let b = make_board(&d);

        assert_eq!(res, b);
        assert_eq!(b.tiles.len(), 25);
    }

    #[test]
    fn test_load_boards() {
        let test_data = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

";
        let res = Board {
            tiles: vec![
                Tile {
                    value: "22",
                    marked: false,
                },
                Tile {
                    value: "13",
                    marked: false,
                },
                Tile {
                    value: "17",
                    marked: false,
                },
                Tile {
                    value: "11",
                    marked: false,
                },
                Tile {
                    value: "0",
                    marked: false,
                },
                Tile {
                    value: "8",
                    marked: false,
                },
                Tile {
                    value: "2",
                    marked: false,
                },
                Tile {
                    value: "23",
                    marked: false,
                },
                Tile {
                    value: "4",
                    marked: false,
                },
                Tile {
                    value: "24",
                    marked: false,
                },
                Tile {
                    value: "21",
                    marked: false,
                },
                Tile {
                    value: "9",
                    marked: false,
                },
                Tile {
                    value: "14",
                    marked: false,
                },
                Tile {
                    value: "16",
                    marked: false,
                },
                Tile {
                    value: "7",
                    marked: false,
                },
                Tile {
                    value: "6",
                    marked: false,
                },
                Tile {
                    value: "10",
                    marked: false,
                },
                Tile {
                    value: "3",
                    marked: false,
                },
                Tile {
                    value: "18",
                    marked: false,
                },
                Tile {
                    value: "5",
                    marked: false,
                },
                Tile {
                    value: "1",
                    marked: false,
                },
                Tile {
                    value: "12",
                    marked: false,
                },
                Tile {
                    value: "20",
                    marked: false,
                },
                Tile {
                    value: "15",
                    marked: false,
                },
                Tile {
                    value: "19",
                    marked: false,
                },
            ],
        };

        let d = test_data
            .lines()
            .map(str::to_string)
            .collect::<Vec<String>>();

        assert_eq!(load_boards(&d), vec![res.clone(), res]);
    }

    #[test]
    fn test_play() {
        let mut b = Board {
            tiles: vec![
                Tile {
                    value: "22",
                    marked: false,
                },
                Tile {
                    value: "13",
                    marked: false,
                },
                Tile {
                    value: "17",
                    marked: false,
                },
                Tile {
                    value: "11",
                    marked: false,
                },
                Tile {
                    value: "0",
                    marked: false,
                },
                Tile {
                    value: "8",
                    marked: false,
                },
                Tile {
                    value: "2",
                    marked: false,
                },
                Tile {
                    value: "23",
                    marked: false,
                },
                Tile {
                    value: "4",
                    marked: false,
                },
                Tile {
                    value: "24",
                    marked: false,
                },
                Tile {
                    value: "21",
                    marked: false,
                },
                Tile {
                    value: "9",
                    marked: false,
                },
                Tile {
                    value: "14",
                    marked: false,
                },
                Tile {
                    value: "16",
                    marked: false,
                },
                Tile {
                    value: "7",
                    marked: false,
                },
                Tile {
                    value: "6",
                    marked: false,
                },
                Tile {
                    value: "10",
                    marked: false,
                },
                Tile {
                    value: "3",
                    marked: false,
                },
                Tile {
                    value: "18",
                    marked: false,
                },
                Tile {
                    value: "5",
                    marked: false,
                },
                Tile {
                    value: "1",
                    marked: false,
                },
                Tile {
                    value: "12",
                    marked: false,
                },
                Tile {
                    value: "20",
                    marked: false,
                },
                Tile {
                    value: "15",
                    marked: false,
                },
                Tile {
                    value: "19",
                    marked: false,
                },
            ],
        };

        assert_eq!(b.play("170"), None);
        assert_eq!(b.play("22"), Some(Outcome::Nothing));
        assert_eq!(b.play("13"), Some(Outcome::Nothing));
        assert_eq!(b.play("17"), Some(Outcome::Nothing));
        assert_eq!(b.play("11"), Some(Outcome::Nothing));
        let o = b.play("0");
        assert!(b.check_horiz(0));
        assert_eq!(o, Some(Outcome::Bingo));
    }

    #[test]
    fn test_horiz() {
        let mut b = Board {
            tiles: vec![
                Tile {
                    value: "22",
                    marked: true,
                },
                Tile {
                    value: "13",
                    marked: true,
                },
                Tile {
                    value: "17",
                    marked: true,
                },
                Tile {
                    value: "11",
                    marked: true,
                },
                Tile {
                    value: "0",
                    marked: false,
                },
                Tile {
                    value: "8",
                    marked: false,
                },
                Tile {
                    value: "2",
                    marked: false,
                },
                Tile {
                    value: "23",
                    marked: false,
                },
                Tile {
                    value: "4",
                    marked: false,
                },
                Tile {
                    value: "24",
                    marked: false,
                },
                Tile {
                    value: "21",
                    marked: false,
                },
                Tile {
                    value: "9",
                    marked: false,
                },
                Tile {
                    value: "14",
                    marked: false,
                },
                Tile {
                    value: "16",
                    marked: false,
                },
                Tile {
                    value: "7",
                    marked: false,
                },
                Tile {
                    value: "6",
                    marked: false,
                },
                Tile {
                    value: "10",
                    marked: false,
                },
                Tile {
                    value: "3",
                    marked: false,
                },
                Tile {
                    value: "18",
                    marked: false,
                },
                Tile {
                    value: "5",
                    marked: false,
                },
                Tile {
                    value: "1",
                    marked: false,
                },
                Tile {
                    value: "12",
                    marked: false,
                },
                Tile {
                    value: "20",
                    marked: false,
                },
                Tile {
                    value: "15",
                    marked: false,
                },
                Tile {
                    value: "19",
                    marked: false,
                },
            ],
        };

        assert!(!b.check_horiz(0));
        assert_eq!(b.play("0"), Some(Outcome::Bingo));
        assert!(b.check_horiz(0));
    }

    #[test]
    fn test_vert() {
        let mut b = Board {
            tiles: vec![
                Tile {
                    value: "22",
                    marked: false,
                },
                Tile {
                    value: "13",
                    marked: false,
                },
                Tile {
                    value: "17",
                    marked: false,
                },
                Tile {
                    value: "11",
                    marked: false,
                },
                Tile {
                    value: "0",
                    marked: true,
                },
                Tile {
                    value: "8",
                    marked: false,
                },
                Tile {
                    value: "2",
                    marked: false,
                },
                Tile {
                    value: "23",
                    marked: false,
                },
                Tile {
                    value: "4",
                    marked: false,
                },
                Tile {
                    value: "24",
                    marked: true,
                },
                Tile {
                    value: "21",
                    marked: false,
                },
                Tile {
                    value: "9",
                    marked: false,
                },
                Tile {
                    value: "14",
                    marked: false,
                },
                Tile {
                    value: "16",
                    marked: false,
                },
                Tile {
                    value: "7",
                    marked: true,
                },
                Tile {
                    value: "6",
                    marked: false,
                },
                Tile {
                    value: "10",
                    marked: false,
                },
                Tile {
                    value: "3",
                    marked: false,
                },
                Tile {
                    value: "18",
                    marked: false,
                },
                Tile {
                    value: "5",
                    marked: true,
                },
                Tile {
                    value: "1",
                    marked: false,
                },
                Tile {
                    value: "12",
                    marked: false,
                },
                Tile {
                    value: "20",
                    marked: false,
                },
                Tile {
                    value: "15",
                    marked: false,
                },
                Tile {
                    value: "19",
                    marked: false,
                },
            ],
        };

        assert!(!b.check_vert(0));
        assert!(!b.check_vert(1));
        assert!(!b.check_vert(2));
        assert!(!b.check_vert(3));
        assert!(!b.check_vert(4));
        b.play("15");
        assert!(!b.check_vert(4));
        b.play("19");
        assert!(b.check_vert(4));
    }

    #[test]
    fn test_case() {
        let test_data = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let mut lines = test_data
            .lines()
            .map(str::to_string)
            .collect::<Vec<String>>();

        let first_line = &lines.drain(0..1).collect::<Vec<String>>()[0];
        assert_eq!(
            first_line,
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
        );
        let moves: Vec<&str> = first_line.split(',').collect();
        assert_eq!(
            moves,
            vec![
                "7", "4", "9", "5", "11", "17", "23", "2", "0", "14", "21", "24", "10", "16", "13",
                "6", "15", "25", "12", "22", "18", "20", "8", "19", "3", "26", "1"
            ]
        );

        let mut boards = load_boards(&lines);
        let winner = find_winner(&moves, &mut boards);

        assert_eq!(winner.0, "24");

        let mut boards = load_boards(&lines);
        let loser = find_loser(&moves, &mut boards);

        assert_eq!(loser.0, "13");
    }
}
