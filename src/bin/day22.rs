//use std::cmp::Ordering;
use std::collections::HashSet;

use aoc2021::lines_as_vec;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let lines = lines_as_vec("input/day22.txt");

    let mut grid = HashSet::new();
    parse(&lines)
        .iter()
        .for_each(|action| run(action, &mut grid));

    grid.len()
}

fn parse(lines: &[String]) -> Vec<(bool, (isize, isize), (isize, isize), (isize, isize))> {
    // on|x|-20||26|y|-36||17|z|-47||7
    //  0 1   2 3 4 5   6 7 8 9  10 11 12
    lines
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(&[' ', '=', '.', ','][..]).collect();
            let op = if parts[0] == "on" { true } else { false };
            let x = (
                parts[2].parse::<isize>().unwrap().max(-50),
                parts[4].parse::<isize>().unwrap().min(50),
            );
            let y = (
                parts[6].parse::<isize>().unwrap().max(-50),
                parts[8].parse::<isize>().unwrap().min(50),
            );
            let z = (
                parts[10].parse::<isize>().unwrap().max(-50),
                parts[12].parse::<isize>().unwrap().min(50),
            );
            (op, x, y, z)
        })
        .collect()
}

fn run(
    action: &(bool, (isize, isize), (isize, isize), (isize, isize)),
    grid: &mut HashSet<(isize, isize, isize)>,
) {
    let (op, x, y, z) = *action;
    for x in x.0..=x.1 {
        for y in y.0..=y.1 {
            for z in z.0..=z.1 {
                if op {
                    grid.insert((x, y, z));
                } else {
                    grid.remove(&(x, y, z));
                }
            }
        }
    }
}

/*
fn partition_run(
    _action: &(bool, (isize, isize), (isize, isize), (isize, isize)),
    _grid: &mut Vec<Vec<(isize, isize)>>,
) {
    //let (op, x, y, z) = *action;
}

fn turn_off(new_off: (isize, isize), axis: &mut Vec<(isize, isize)>) {
    match axis.iter().position(|&(_on, off)| new_off.0 < off) {
        Some(idx) => unsafe {
            let old_on = axis[idx];
            if new_off.1 < old_on.0 {
                // entirely before
                // do nothing
            } else if new_off.0 < old_on.0 && new_off.1 > old_on.0 && new_off.1 < old_on.1 {
                // chops off front
                // overlaps left, cutting off the front
                *axis.get_unchecked_mut(idx) = (new_off.1, old_on.1);
            } else if new_off.0 > old_on.0 && new_off.1 < old_on.1 {
                // entirely within, cutting out the middle
                axis.remove(idx);
                axis.insert(idx, (new_off.1, old_on.1));
                axis.insert(idx, (old_on.0, new_off.0));
            } else if new_off.0 > old_on.0 && new_off.0 < old_on.1 && new_off.1 > old_on.1 {
                // overlaps right, chopping off the end
                *axis.get_unchecked_mut(idx) = (old_on.0, new_off.0);
            } else {
                unreachable!();
            }
        },
        None => {
            // nothing to do
        }
    }
}

// -10on, 0off, 5on, 10off  <-- turn on -3,0
// -10on, -3on, 0off, 0on, 5on, 10off  <-- insert -3 on
// -10on, 0off, 5on, 10off  <-- gets deduped
// -10on, -3on, 0off, 0off, 5on, 10off  <-- insert 0off

// -10on, 0off, 5on, 10off  <-- turn on -3,6
// -10on, -3on, 0off, 0on, 5on, 10off  <-- insert -3 on
// -10on, 0off, 5on, 10off  <-- gets deduped
// -10on, -3on, 0off, 5on, 10off
//

fn turn_on(spot: (isize, isize), axis: &mut Vec<(isize, bool)>) {
    axis.push((spot.0, true));
    if axis.is_empty() {
        axis.push((spot.1, false));
        return;
    }

    axis.sort_unstable_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => match (a.1, b.1) {
            (true, true) | (false, false) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
        },
        o @ _ => o,
    });
    axis.dedup_by(|a, b| match (a.1, b.1) {
        (true, true) | (false, false) => true,
        (false, true) => a.0 == b.0,
        (true, false) => false,
    });

    let started = axis.iter().position(|s| s.0 == spot.0).unwrap();
    let mut til = axis.iter().position(|s| spot.1 >= s.0).unwrap();
    if !axis[til].1 {
        axis.insert(til + 1, (spot.1, false));
        til = til + 1;
    }
    axis.drain(started..til);
}
*/

#[cfg(test)]
mod day22_tests {

    use super::*;
    use aoc2021::str_as_vec;

    fn test_turn_on() {
        let mut axis = vec![(-10, true), (10, false)];
        //turn_on((20, 30), &mut axis);
        assert_eq!(
            vec![(-10, true), (10, false), (20, true), (30, false)],
            axis
        );
    }

    fn small_test_case() {
        let test_data = str_as_vec(
            "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
        );

        let mut grid = HashSet::new();

        let actions = parse(&test_data);

        run(&actions[0], &mut grid);
        assert_eq!(27, grid.len());

        run(&actions[1], &mut grid);
        assert_eq!(27 + 19, grid.len());

        run(&actions[2], &mut grid);
        assert_eq!(27 + 19 - 8, grid.len());

        run(&actions[3], &mut grid);
        assert_eq!(39, grid.len());
    }

    //#[test]
    fn lg_test_case() {
        let test_data = str_as_vec(
            "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682",
        );

        let mut grid = HashSet::new();
        parse(&test_data)
            .iter()
            .for_each(|action| run(action, &mut grid));

        assert_eq!(590784, grid.len());
    }
}
