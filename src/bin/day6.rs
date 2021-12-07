use aoc2021::lines_as_vec;
use std::collections::VecDeque;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let data = lines_as_vec("input/day6.txt");
    let st = get_initial_state(&data);

    let mut world: Vec<Fish> = st.iter().map(|&s| Fish(s)).collect();
    for _ in 0..80 {
        let mut t = tick(&mut world);
        world.append(&mut t);
    }

    world.len()
}

fn part2() -> usize {
    let data = lines_as_vec("input/day6.txt");
    let fish = get_initial_state(&data);

    let rate = 7; // fish reproduce after 7 days
    let juv = 2; // fish need two extra days the first cycle

    let days = 256;

    fish_life(rate, juv, days, &fish) as usize

}

fn get_initial_state(lines: &[String]) -> Vec<i64> {
    lines[0]
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn tick(world: &mut [Fish]) -> Vec<Fish> {
    world.iter_mut().map(|fish| fish.tick()).flatten().collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Fish(i64);

impl Default for Fish {
    fn default() -> Self {
        Fish(8)
    }
}

impl Fish {
    fn tick(&mut self) -> Option<Self> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Fish::default())
        } else {
            self.0 -= 1;
            None
        }
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn test_live() {
        let mut f = vec![Fish(0)];
        let r = tick(&mut f);
        assert_eq!(f, vec![Fish(6)]);
        assert_eq!(r, vec![Fish(8)]);
    }

    #[test]
    fn test_case() {
        let test_data = vec!["3,4,3,1,2".to_owned()];
        let st = get_initial_state(&test_data);

        let mut world: Vec<Fish> = st.iter().map(|&s| Fish(s)).collect();
        for _ in 0..18 {
            let mut t = tick(&mut world);
            world.append(&mut t);
        }

        assert_eq!(26, world.len());

        let mut world: Vec<Fish> = st.iter().map(|&s| Fish(s)).collect();
        for _ in 0..80 {
            let mut t = tick(&mut world);
            world.append(&mut t);
        }

        assert_eq!(5934, world.len());
    }

    #[test]
    fn test_case_part_2() {
        let test_data = vec!["3,4,3,1,2".to_owned()];

        let rate = 7; // fish reproduce after 7 days
        let juv = 2; // fish need two extra days the first cycle

        let fish: Vec<i64> = get_initial_state(&test_data);

        let days = 18;
        assert_eq!(26, fish_life(rate, juv, days, &fish));

        let days = 80;
        assert_eq!(5934, fish_life(rate, juv, days, &fish));

        let days = 256;
        assert_eq!(26984457539, fish_life(rate, juv, days, &fish));
    }
}

fn fish_life(rate: i64, juv: i64, days: i64, fish: &[i64]) -> i64 {
    let mut world = VecDeque::from(vec![0; (rate + juv) as usize]);

    for &f in fish {
        world[f as usize] += 1;
    }

    for _ in 0..days {
        let spawned = world.pop_front().unwrap();
        world.push_back(spawned);
        world[rate as usize - 1] += spawned;
    }

    world.iter().sum()
}
