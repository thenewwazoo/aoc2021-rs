use aoc2021::lines_as_vec;

use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let mut grid = lines_to_grid(&lines_as_vec("input/day11.txt"));
    let mut count = 0;

    for _ in 0..=99 {
        // why 99? should be 100, no?
        let r = step_grid(&grid);
        grid = r.0;
        count += r.1;
    }
    count
}

fn part2() -> usize {
    let mut grid = lines_to_grid(&lines_as_vec("input/day11.txt"));
    let mut iter = 0;

    loop {
        let r = step_grid(&grid);
        grid = r.0;
        iter += 1;
        if is_all_flash(&grid) {
            break;
        }
    }
    iter
}

fn is_all_flash(grid: &GridMap) -> bool {
    grid.iter().all(|(_, &v)| v == 0)
}

type GridMap = HashMap<(usize, usize), u8>;

fn lines_to_grid(lines: &[String]) -> GridMap {
    lines
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| ((j, i), c))
                .collect::<Vec<((usize, usize), char)>>()
        })
        .map(|((x, y), c): ((usize, usize), char)| {
            ((x, y), u8::try_from(c.to_digit(10).unwrap()).unwrap())
        })
        .collect()
}

fn step_grid(grid: &GridMap) -> (GridMap, usize) {
    let mut res = inc_grid(grid);
    let mut flash_cnt = 0;

    loop {
        let flashes = find_flashes(&res);
        flash_cnt += flashes.len();
        if flashes.is_empty() {
            break;
        }
        apply_flashes(&mut res, flashes);
    }

    (res, flash_cnt)
}

fn inc_grid(grid: &GridMap) -> GridMap {
    grid.iter().map(|(&k, d)| (k, d + 1)).collect()
}

fn find_flashes(g: &GridMap) -> Vec<(usize, usize)> {
    g.iter().filter(|(_, &d)| d > 9).map(|(&k, &_)| k).collect()
}

fn apply_flashes(g: &mut GridMap, flashes: Vec<(usize, usize)>) {
    flashes.iter().for_each(|&i| {
        apply_flash(g, i);
    });
}

fn apply_flash(g: &mut GridMap, loc: (usize, usize)) {
    match g.get_mut(&loc) {
        Some(v) => *v = 0,
        _ => panic!("f"),
    };
    inc_surr(g, loc);
}

fn inc_surr(g: &mut GridMap, i: (usize, usize)) {
    for a in [
        (-1isize, -1isize),
        (-1isize, 0),
        (-1isize, 1),
        (0, -1isize),
        (0, 1),
        (1, -1isize),
        (1, 0),
        (1, 1),
    ] {
        if let (Some(x), Some(y)) = (
            if a.0 < 0 {
                i.0.checked_sub(1)
            } else {
                i.0.checked_add(a.0 as usize)
            },
            if a.1 < 0 {
                i.1.checked_sub(1)
            } else {
                i.1.checked_add(a.1 as usize)
            },
        ) {
            if let Some(v) = g.get_mut(&(x, y)) {
                if *v != 0 {
                    *v += 1;
                    //*v = u8::min(9, *v + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    use aoc2021::str_as_vec;

    #[test]
    fn test_apply_flash() {
        let test_data = "11111
19991
19191
19991
11111";
        let lines = str_as_vec(test_data);
        let mut grid = inc_grid(&lines_to_grid(&lines));

        apply_flash(&mut grid, (1, 1));

        println!("{:?}", grid);
        assert_eq!(Some(&0), grid.get(&(1, 1)));
        assert_eq!(Some(&3), grid.get(&(0, 0)));
        assert_eq!(Some(&3), grid.get(&(2, 2)));
    }

    #[test]
    fn test_find_flashes() {
        let test_data = "11111
19991
19191
19991
11111";
        let lines = str_as_vec(test_data);
        let grid = inc_grid(&lines_to_grid(&lines));

        let mut res = vec![
            (1, 1),
            (2, 1),
            (3, 1),
            (1, 2),
            (3, 2),
            (1, 3),
            (2, 3),
            (3, 3),
        ];
        res.sort_unstable();

        let mut found = find_flashes(&grid);
        found.sort_unstable();

        assert_eq!(res, found);
    }

    #[test]
    fn test_step_grid() {
        let test_data = "11111
19991
19191
19991
11111";
        let lines = str_as_vec(test_data);
        let grid = lines_to_grid(&lines);

        let (r, c) = step_grid(&grid);

        let res_str = "34543
40004
50005
40004
34543";
        let res_grid = lines_to_grid(&str_as_vec(res_str));

        assert_eq!(res_grid, r);
        assert_eq!(9, c);
    }

    #[test]
    fn test_inc_surr() {
        let mut g: GridMap = [
            ((0, 0), 1),
            ((0, 1), 1),
            ((0, 2), 1),
            ((1, 0), 1),
            ((1, 1), 1),
            ((1, 2), 1),
            ((2, 0), 1),
            ((2, 1), 1),
            ((2, 2), 1),
        ]
        .into_iter()
        .collect();

        inc_surr(&mut g, (1, 1));
        assert_eq!(2, *g.get(&(0, 0)).unwrap());
        assert_eq!(2, *g.get(&(0, 1)).unwrap());
        assert_eq!(2, *g.get(&(0, 2)).unwrap());
        assert_eq!(2, *g.get(&(1, 0)).unwrap());
        assert_eq!(1, *g.get(&(1, 1)).unwrap());
        assert_eq!(2, *g.get(&(1, 2)).unwrap());
        assert_eq!(2, *g.get(&(2, 0)).unwrap());
        assert_eq!(2, *g.get(&(2, 1)).unwrap());
        assert_eq!(2, *g.get(&(2, 2)).unwrap());
    }

    #[test]
    fn test_case() {
        let test_data = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let lines = str_as_vec(test_data);
        let mut grid = lines_to_grid(&lines);
        let mut count = 0;

        let after_step1 = lines_to_grid(&str_as_vec(
            "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637",
        ));

        let r = step_grid(&grid);
        grid = r.0;
        count += r.1;

        assert_eq!(after_step1, grid);
        assert_eq!(0, count);

        let after_step2 = lines_to_grid(&str_as_vec(
            "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848",
        ));

        let r = step_grid(&grid);
        grid = r.0;
        count += r.1;

        assert_eq!(after_step2, grid);
        assert_eq!(35, count);

        for _ in 3..=10 {
            let r = step_grid(&grid);
            grid = r.0;
            count += r.1;
        }

        let after_step10 = lines_to_grid(&str_as_vec(
            "0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000",
        ));

        assert_eq!(after_step10, grid);
        assert_eq!(204, count);
    }
}
