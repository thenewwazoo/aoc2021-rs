use aoc2021::lines_as_vec;

use std::collections::{HashMap, VecDeque};

fn main() {
    println!("{}", part1());
}

fn part1() -> usize {
    let tgt = parse_line(&lines_as_vec("input/day17.txt")[0]);
    println!("tgt is {:?}", tgt);

    //let mut solutions = HashMap::new();
    let mut solutions = 0;

    let mut iters = 0;
    /*
    let mut vq = VecDeque::new();
    vq.push_back((1, 1));


    while !vq.is_empty() && iters < 10000 {
        let v = vq.pop_front().unwrap();
        vq.extend(next_v(v));
    */

    for x in 1i32..tgt.t_r.0 {
        for y in (tgt.b_l.1)..tgt.b_l.1.abs() {
            let v = (x, y);
            let mut world = World::new(&tgt, v);
            let (_steps, result) = run_world(&mut world);
            //println!("{:?}", steps);
            match result {
                Position::Inside => {
                    //solutions.insert(v, max_height(&steps));
                    solutions += 1;
                }
                //Position::Short => continue,
                Position::Past | Position::Short => {
                    iters += 1;
                    continue;
                }
                _ => unreachable!(),
            };
        }
    }

    println!("{} iters", iters);
    /*
    for (&k, &v) in solutions.iter() {
        println!("{:?} - {:?}", k, v);
    }
    */
    solutions //.len()
              //println!("\n\n{:?}", solutions);
}

fn next_v(v: (i32, i32)) -> Vec<(i32, i32)> {
    if v.0 == v.1 {
        vec![(v.0, v.1 + 1), (v.0 + 1, v.1 + 1), (v.0 + 1, v.1)]
    } else if v.0 > v.1 {
        vec![(v.0 + 1, v.1)]
    } else {
        vec![(v.0, v.1 + 1)]
    }
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
    fn check_past(&self, probe: &Probe) -> Option<Position> {
        if probe.v.0 >= 0 && probe.p.0 > self.t_r.0 {
            // moving right, past the right edge
            // println!("past the right edge");
            Some(Position::Past)
        } else if probe.v.0 <= 0 && probe.p.0 < self.t_l.0 {
            // moving left, past the left edge
            // println!("past the left edge");
            Some(Position::Short)
        } else if probe.v.1 <= 0 && probe.p.1 <= self.b_l.1 {
            // moving down, past the bottom
            // println!("past the bottom");
            Some(Position::Short)
        } else {
            None
        }
    }

    fn check_transit(&self, pre: &Probe, post: &Probe) -> Option<Position> {
        match [self.t_l, self.b_l, self.b_r, self.t_r, self.t_l]
            .windows(2) // get every edge around the target
            .map(|w| Target::intersect(pre.p, post.p, w[0], w[1])) // find any that intersect our path
            .fold(0u8, |acc, i| acc + i as u8)
        {
            2 => {
                // println!("shot through it");
                Some(Position::Past) // shot straight through it
            }
            1 => {
                // println!("landed inside it");
                Some(Position::Inside) // landed inside it
            }
            _ => None,
        }
    }

    fn check_colinear(&self, probe: &Probe) -> bool {
        if (probe.p.0 == self.b_l.0 || probe.p.0 == self.b_r.0)
            && probe.p.1 < self.t_l.1
            && probe.p.1 > self.b_l.1
        {
            // println!("on left/right edge");
            true
        } else if (probe.p.1 == self.t_l.1 || probe.p.1 == self.b_l.1)
            && probe.p.0 < self.t_r.0
            && probe.p.0 > self.t_l.0
        {
            // println!("on top/bottom edge");
            true
        } else {
            false
        }
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
    Short,
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

        match self.target.check_transit(&pre, &self.probe) {
            Some(Position::Past) => {
                if self.target.check_colinear(&self.probe) {
                    Position::Inside
                } else {
                    Position::Past
                }
            }
            Some(Position::Inside) => Position::Inside,
            None => {
                if let Some(p) = self.target.check_past(&self.probe) {
                    p // we're past the sides and moving away, or off the bottom and dropping
                } else {
                    Position::Before
                }
            }
            _ => unreachable!(),
        }

        /*
        match (
            self.target.check_past(&self.probe),
            self.target.check_transit(&pre, &self.probe),
            self.target.check_colinear(&self.probe),
        ) {
            (Some(Position::Short), _, _) => Position::Short,
            (Some(Position::Past), _, false) | (None, Some(Position::Past), _) => Position::Past,
            (Some(_), _, true) | (None, None, true) => Position::Inside,
            (None, None, false) => Position::Before,
            (None, Some(Position::Inside), _) => Position::Inside,
            (None, Some(Position::Before), _) => unreachable!(),
        }
        */
    }
}

fn run_world(world: &mut World) -> (Vec<(i32, i32)>, Position) {
    let mut steps = vec![world.probe.p];
    let result = loop {
        match world.step() {
            Position::Before => steps.push(world.probe.p),
            p @ _ => break p,
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
        let t = Target {
            t_l: (20, -5),
            t_r: (30, -5),
            b_l: (20, -10),
            b_r: (30, -10),
        };

        assert_eq!(
            None,
            t.check_past(&Probe {
                p: (0, 0),
                v: (7, 2)
            })
        );
        assert_eq!(
            Some(Position::Short),
            t.check_past(&Probe {
                p: (0, 0),
                v: (0, 2)
            })
        );
        assert_eq!(
            Some(Position::Past),
            t.check_past(&Probe {
                p: (31, 0),
                v: (7, 2)
            })
        );
        assert_eq!(
            Some(Position::Short),
            t.check_past(&Probe {
                p: (1, -10),
                v: (7, -2)
            })
        );
    }

    #[test]
    fn test_case() {
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

        let mut world = World::new(&tgt, (6, 3));

        let (steps, result) = run_world(&mut world);

        assert_eq!(Position::Inside, result);
        assert_eq!((21, -9), world.probe.p);
        assert_eq!(10, steps.len());

        let mut world = World::new(&tgt, (17, -4));

        let (steps, result) = run_world(&mut world);

        assert_eq!(Position::Past, result);
        assert_eq!((33, -9), world.probe.p);
        assert_eq!(3, steps.len());

        let mut world = World::new(&tgt, (6, 9));

        let (steps, result) = run_world(&mut world);

        assert_eq!(Position::Inside, result);
        assert_eq!(45, max_height(&steps));
    }
}
