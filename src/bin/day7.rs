use aoc2021::{get_num_list, lines_as_vec};

fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> i64 {
    // okay, listen. so for part 1, I wrote this to use the median. and it worked. cool.
    //
    // but it didn't work for part 2. but I had already written some test code that just did a
    // search. because why not brute-force it, right? turns out the totally naive, brute-force
    // solution runs in some tiny fraction of a second when compiled in release mode. so fuck it,
    // we'll just brute-force it. this *should* use the mean to calculate it directly, but...
    // whatever.

    let mut initial_state = get_num_list(&lines_as_vec("input/day7.txt"));
    initial_state.sort();
    (*initial_state.first().unwrap()..=*initial_state.as_slice().last().unwrap())
        .map(|dest| calc_fuel_for(&initial_state, dest))
        .min()
        .unwrap()
}

fn calc_fuel_for(posns: &[i64], dest: i64) -> i64 {
    posns
        .iter()
        .map(|&p| (1..=(p - dest).abs()).sum::<i64>())
        .sum()
}

#[cfg(test)]
mod day7_tests {

    use super::*;

    #[test]
    fn test_case() {
        let test_data = vec![String::from("16,1,2,0,4,2,7,1,2,14")];
        let mut initial_state = get_num_list::<i64>(&test_data);

        let c = (initial_state.iter().cloned().min().unwrap()
            ..=initial_state.iter().cloned().max().unwrap())
            .map(|dest| calc_fuel_for(&initial_state, dest))
            .min()
            .unwrap();

        assert_eq!(c, 168);
    }
}
