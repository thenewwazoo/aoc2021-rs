use std::collections::HashMap;

fn main() {
    println!("{}", part1());
}

fn part1() -> usize {
    0
}

/*
 * #############
 * #01x2x3x4x56#
 * ###7#9#B#D###
 *   #8#A#C#E#
 *   #########
 */
fn make_adj_map() -> HashMap<usize, Vec<usize>> {
    HashMap::from([
        (0, vec![1]),
        (1, vec![2, 7]),
        (7, vec![8]),
        (2, vec![3, 9]),
        (9, vec![0xA]),
        (3, vec![4, 0xB]),
        (0xB, vec![0xC]),
        (4, vec![5, 0xD]),
        (0xD, vec![0xE]),
        (5, vec![6]),
    ])
}

// NPC IDs: As = 0,1, Bs = 2,3, ...
// this takes which id is in which named map loc
fn make_loc_map(
    seven: usize,
    eight: usize,
    nine: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
) -> HashMap<usize, usize> {
    HashMap::from([
        (seven, 7),
        (eight, 8),
        (nine, 9),
        (a, 0xA),
        (b, 0xB),
        (c, 0xC),
        (d, 0xD),
        (e, 0xE),
    ])
}

/*
 * #############
 * #01x2x3x4x56#
 * ###7#9#B#D###
 *   #8#A#C#E#
 *   #########
 */
fn get_legal_moves(
    p: usize,
    locs: HashMap<usize, usize>,
    map: HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    // first, get all possibilities
}
