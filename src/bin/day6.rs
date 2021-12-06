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

fn get_initial_state(lines: &[String]) -> Vec<u64> {
    lines[0]
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn tick(world: &mut [Fish]) -> Vec<Fish> {
    world.iter_mut().map(|fish| fish.tick()).flatten().collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Fish(u64);

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

        let mut world: Vec<Fish> = st.iter().map(|&s| Fish(s)).collect();
        for _ in 0..256 {
            let mut t = tick(&mut world);
            world.append(&mut t);
        }

        assert_eq!(26984457539, world.len());
    }
}
