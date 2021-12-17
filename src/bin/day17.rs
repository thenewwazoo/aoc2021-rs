use aoc2021::lines_as_vec;

fn main() {
    println!("{}", part1());
}

fn part1() -> i32 {
    let tgt = parse_line(&lines_as_vec("input/day17.txt")[0]);
    0
}

fn parse_line(line: &str) -> Target {
    // target area: x=20..30, y=-10..-5
    let words = line.split(&[',', '=', '.'][..]).collect::<Vec<_>>();
    let left = words[1].parse::<i32>().unwrap();
    let right = words[3].parse::<i32>().unwrap();
    let bottom = words[5].parse::<i32>().unwrap();
    let top = words[7].parse::<i32>().unwrap();
    Target {
        t_l: (left, top),
        t_r: (right, top),
        b_l: (left, bottom),
        b_r: (right, bottom),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Probe {
    p: (i32, i32),
    v: (i32, i32),
}

impl Probe {
    fn tick(&mut self) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        if self.v.0 > 0 {
            self.v.0 -= 1;
        } else if self.v.0 < 0 {
            self.v.0 += 1;
        }
        self.v.1 -= 1;
    }

    fn ccw(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
        (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Target {
    t_l: (i32, i32),
    b_l: (i32, i32),
    t_r: (i32, i32),
    b_r: (i32, i32),
}

impl Target {
    fn check_past(&self, probe: &Probe) -> bool {
        println!("{:?} {:?}", self, probe);
        if probe.v.0 >= 0 && probe.p.0 > self.t_r.0 {
            // moving right, past the right edge
            println!("past the right edge");
            true
        } else if probe.v.0 <= 0 && probe.p.0 < self.t_l.0 {
            // moving left, past the left edge
            println!("past the left edge");
            true
        } else if probe.v.1 < 0 && probe.p.1 <= self.b_l.1 {
            // moving down, past the bottom
            println!("past the bottom");
            true
        } else {
            false
        }
    }

    fn check_transit(&self, pre: &Probe, post: &Probe) -> Option<Position> {
        match [self.t_l, self.b_l, self.b_r, self.t_r, self.t_l]
            .windows(2) // get every edge around the target
            .map(|w| Target::intersect(pre.p, post.p, w[0], w[1])) // find any that intersect our path
            .fold(0u8, |acc, i| acc + i as u8)
        {
            2 => Some(Position::Past),   // shot straight through it
            1 => Some(Position::Inside), // landed inside it
            _ => None,
        }
    }

    fn check_colinear(&self, probe: &Probe) -> bool {
        ((probe.p.0 == self.b_l.0 || probe.p.0 == self.b_r.0)
            && probe.p.1 < self.t_l.1
            && probe.p.1 > self.b_l.1)
            || ((probe.p.1 == self.t_l.1 || probe.p.1 == self.b_l.1)
                && probe.p.0 < self.t_r.0
                && probe.p.0 > self.t_l.0)
    }

    fn intersect(t1: (i32, i32), t2: (i32, i32), p1: (i32, i32), p2: (i32, i32)) -> bool {
        Probe::ccw(t1, p1, p2) != Probe::ccw(t2, p1, p2)
            && Probe::ccw(t1, t2, p1) != Probe::ccw(t1, t2, p2)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Position {
    Before,
    Inside,
    Past,
}

#[derive(Debug)]
struct World {
    target: Target,
    probe: Probe,
}

impl World {
    fn new(tgt: &Target, p_v: (i32, i32)) -> Self {
        World {
            target: tgt.clone(),
            probe: Probe {
                p: (0, 0),
                v: (p_v.0, p_v.1),
            },
        }
    }

    fn step(&mut self) -> Position {
        let pre = self.probe.clone();
        self.probe.tick();

        match (
            self.target.check_past(&self.probe),
            self.target.check_transit(&pre, &self.probe),
            self.target.check_colinear(&self.probe),
        ) {
            (true, _, false) | (false, Some(Position::Past), _) => Position::Past,
            (true, _, true) | (false, None, true) => Position::Inside,
            (false, None, false) => Position::Before,
            (false, Some(Position::Inside), _) => Position::Inside,
            (false, Some(Position::Before), _) => unreachable!(),
        }
    }
}

fn run_world(world: &mut World) -> (Vec<(i32, i32)>, Position) {
    let mut steps = vec![world.probe.p];
    let result = loop {
        println!("{:?}", world);
        match world.step() {
            Position::Before => steps.push(world.probe.p),
            Position::Past => break Position::Past,
            Position::Inside => break Position::Inside,
        }
    };
    steps.push(world.probe.p);
    (steps, result)
}

fn max_height(steps: &[(i32, i32)]) -> i32 {
    steps.iter().map(|p| p.1).max().unwrap()
}

#[cfg(test)]
mod day17_tests {

    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "target area: x=20..30, y=-10..-5";
        let tgt = parse_line(line);

        assert_eq!(
            Target {
                t_l: (20, -5),
                t_r: (30, -5),
                b_l: (20, -10),
                b_r: (30, -10),
            },
            tgt
        );
    }

    #[test]
    fn test_check_past() {
        let world = World::new(
            &Target {
                t_l: (20, -5),
                t_r: (30, -5),
                b_l: (20, -10),
                b_r: (30, -10),
            },
            (7, 2),
        );
        let t = world.target;

        assert!(!t.check_past(&Probe {
            p: (0, 0),
            v: (7, 2)
        }));
        assert!(t.check_past(&Probe {
            p: (0, 0),
            v: (-7, 2)
        }));
        assert!(t.check_past(&Probe {
            p: (31, 0),
            v: (7, 2)
        }));
        assert!(t.check_past(&Probe {
            p: (1, -10),
            v: (7, -2)
        }));
    }

    #[test]
    fn test_case() {
        println!("\nstarting at (7, 2)");
        let tgt = Target {
            t_l: (20, -5),
            t_r: (30, -5),
            b_l: (20, -10),
            b_r: (30, -10),
        };
        let mut world = World::new(&tgt, (7, 2));

        let (steps, result) = run_world(&mut world);

        assert_eq!(Position::Inside, result);
        assert_eq!((28, -7), world.probe.p);
        assert_eq!(8, steps.len());
        assert_eq!(3, max_height(&steps));

        println!("\nstarting at (6, 3)");
        let mut world = World::new(&tgt, (6, 3));

        let (steps, result) = run_world(&mut world);

        assert_eq!(Position::Inside, result);
        assert_eq!((21, -9), world.probe.p);
        assert_eq!(10, steps.len());

        println!("\nstarting at (17, -4)");
        let mut world = World::new(&tgt, (17, -4));

        let (steps, result) = run_world(&mut world);

        assert_eq!(Position::Past, result);
        assert_eq!((33, -9), world.probe.p);
        assert_eq!(3, steps.len());

        println!("\nstarting at (6, 9)");
        let mut world = World::new(&tgt, (6, 9));

        let (steps, result) = run_world(&mut world);

        assert_eq!(Position::Inside, result);
        assert_eq!(45, max_height(&steps));
    }
}
