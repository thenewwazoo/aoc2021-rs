use aoc2021::lines_as_vec;

fn main() {
    println!("Part 1: {}", part1());
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
    9
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

        /*
        let mut count = 1;
        let days = 7;
        let fish = vec![6];

        assert_eq!(2, fish.len() as i64 + fish_life(rate, juv, days, &fish));

        let mut count = 1;
        let days = 14;
        let fish = vec![6];

        assert_eq!(3, fish.len() as i64 + fish_life(rate, juv, days, &fish));
        */

        let fish: Vec<i64> = get_initial_state(&test_data);
        let days = 18;

        assert_eq!(26, fish.len() as i64 + fish_life(rate, juv, days, &fish));

        let days = 80;

        assert_eq!(5934, fish.len() as i64 + fish_life(rate, juv, days, &fish));
    }
}

fn fish_life(rate: i64, juv: i64, days: i64, fish: &[i64]) -> i64 {
    let mut offspring = 0;
    for age in fish.iter() {
        println!("first fish is {} old", age);
        offspring += spawns(rate, juv, days, 0 + age);
    }
    offspring
}

fn spawns(rate: i64, juv: i64, lim: i64, today: i64) -> i64 {
    println!("spawning at day {} ({} days remain)", today, lim - today);
    if today + rate > lim {
        println!("not enough time to spawn again");
        1
    } else {
        let mut count = 0;
        let o = (lim - today) / rate;
        println!("will spawn {} more times", o);
        for f in 1..=o {
            let next_spawn = today + (f*rate) + 2;
            count += spawns(rate, juv, lim, next_spawn);
        }
        count
    }
}
