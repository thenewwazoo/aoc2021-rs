use aoc2021::lines_as_vec;

/*
 * v = < max(0, |x_0|-t), y_0 - t >
 * p_t+1 = < x_t + max(0, |x_0|-t), y_t + y_0 - t >
 */

use std::ops::RangeInclusive;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> i32 {
    let tgt = parse_line(&lines_as_vec("input/day17.txt")[0]);
    ((tgt.b_y.abs() - 1) * tgt.b_y.abs()) / 2
}

fn part2() -> i32 {
    let tgt = parse_line(&lines_as_vec("input/day17.txt")[0]);
    let mut solns = 0;
    println!("{:?}", tgt);
    for v_x in 0..=tgt.r_x {
        for v_y in tgt.b_y..tgt.b_y.abs() {
            let mut v_x = v_x;
            let mut v_y = v_y;
            let mut x = 0;
            let mut y = 0;
            for _ in 0..3000 {
                x += v_x;
                y += v_y;
                v_x = if v_x > 0 { v_x - 1 } else { 0 };
                v_y = v_y - 1;
                if x >= tgt.l_x && x <= tgt.r_x && y <= tgt.t_y && y >= tgt.b_y {
                    solns += 1;
                    break;
                }
            }
        }
    }
    solns
}

fn solve_tri(n: i32) -> Option<i32> {
    let s = f64::sqrt((2.0 * f64::from(n)) + 0.25) - 0.5;
    if s.fract() == 0.0 {
        Some(unsafe { f64::to_int_unchecked::<i32>(s) })
    } else {
        None
    }
}

fn find_lowest_tri(from: i32, to: i32) -> Option<i32> {
    let mut i = 0;
    loop {
        i += 1;
        let r = i * (i + 1) / 2;
        if r > from {
            break Some(i);
        }
        if r > to {
            break None;
        }
    }
}

fn parse_line(line: &str) -> Target {
    // target area: x=20..30, y=-10..-5
    let words = line.split(&[',', '=', '.'][..]).collect::<Vec<_>>();
    let l_x = words[1].parse::<i32>().unwrap();
    let r_x = words[3].parse::<i32>().unwrap();
    let b_y = words[5].parse::<i32>().unwrap();
    let t_y = words[7].parse::<i32>().unwrap();
    Target { l_x, r_x, b_y, t_y }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Target {
    l_x: i32,
    r_x: i32,
    b_y: i32,
    t_y: i32,
}

#[cfg(test)]
mod day17_tests {

    use super::*;

    const TEST_DATA: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part2() {
        let tgt = parse_line(TEST_DATA);

        // probes fired upward fall back down with v_neg @ h_0 = -v_0-1
        let up_y_velocities: Vec<i32> = (0..=(tgt.b_y.abs() - 1)).collect();
        println!("{:?}", up_y_velocities);

        // probes fired downward so they just hit the target range immediately
        let direct_y_velocities: Vec<i32> = (tgt.b_y..=tgt.t_y).collect();
        println!("{:?}", direct_y_velocities);

        // probes fired downward with increasing velocity until they miss
        //let accelerating_y_velocities: Vec<i32> = (0..=
        println!("adsf {}", find_lowest_tri(tgt.b_y, tgt.t_y).unwrap());
        let dropped_y_velocities: Vec<i32> = (find_lowest_tri(tgt.b_y, tgt.t_y).unwrap()
            ..find_lowest_tri(tgt.b_y - 100, tgt.b_y).unwrap())
            .collect();
        println!("{:?}", dropped_y_velocities);

        let slowing_x_velocities: Vec<i32> =
            ((find_lowest_tri(tgt.l_x, tgt.r_x).unwrap())..=(tgt.r_x / 2)).collect();
        println!("{:?}", slowing_x_velocities);

        let direct_x_velocities: Vec<i32> = (tgt.l_x..=tgt.r_x).collect();
        println!("{:?}", direct_x_velocities);

        assert!(false);
    }

    #[test]
    fn test_find_tris() {
        assert_eq!(Some(6), find_lowest_tri(20, 21));
    }

    #[test]
    fn test_part1() {
        let tgt = parse_line(TEST_DATA);

        assert_eq!(45, ((tgt.b_y.abs() - 1) * tgt.b_y.abs()) / 2);
    }
}
