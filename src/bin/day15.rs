use aoc2021::lines_as_vec;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let (map, width, depth) = parse(&lines_as_vec("input/day15.txt"));

    shortest_path(&map, 0, width * depth - 1).unwrap()
}

fn part2() -> usize {
    let (map, width, depth) = wrapped_parse(&lines_as_vec("input/day15.txt"));

    shortest_path(&map, 0, width * depth - 1).unwrap()
}

// this implementation of dijkstra gleefully cribbed from https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

#[derive(Debug, PartialEq)]
struct Edge {
    node: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(adj_list: &AdjList, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

type AdjList = Vec<Vec<Edge>>;

fn parse(lines: &[String]) -> (AdjList, usize, usize) {
    let width = lines[0].len();
    let depth = lines.len();
    let values: Vec<usize> = lines
        .iter()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).expect("bad digit") as usize)
        .collect();

    (
        (0..(values.len()))
            .into_iter()
            .map(|i| make_adj(&values, i, width, depth))
            .collect(),
        width,
        depth,
    )
}

fn make_adj(costs: &[usize], i: usize, width: usize, depth: usize) -> Vec<Edge> {
    let mut edges = Vec::new();
    if i >= width * depth {
        panic!("out of bounds");
    }

    // right
    if i % width < width - 1 {
        edges.push(Edge {
            node: i + 1,
            cost: costs[i + 1],
        });
    }
    // left
    if i > 0 && i % width > 0 {
        edges.push(Edge {
            node: i - 1,
            cost: costs[i - 1],
        });
    }
    // down
    if i + width < width * depth {
        edges.push(Edge {
            node: i + width,
            cost: costs[i + width],
        });
    }
    // up
    if i >= width {
        edges.push(Edge {
            node: i - width,
            cost: costs[i - width],
        });
    }
    edges
}

fn wrapped_parse(lines: &[String]) -> (AdjList, usize, usize) {
    let width = lines[0].len() * 5;
    let depth = lines.len() * 5;
    let wide: Vec<usize> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .flat_map(|v| {
            (0..5).fold(Vec::new(), |mut acc, i| {
                acc.append(
                    &mut v
                        .iter()
                        .map(|&d| if d + i > 9 { d + i - 9 } else { d + i })
                        .collect::<Vec<usize>>(),
                );
                acc
            })
        })
        .collect();

    let values = (0..5).fold(Vec::new(), |mut acc, i| {
        acc.append(
            &mut wide
                .iter()
                .map(|&d| if d + i > 9 { d + i - 9 } else { d + i })
                .collect::<Vec<usize>>(),
        );
        acc
    });

    (
        (0..(values.len()))
            .into_iter()
            .map(|i| make_adj(&values, i, width, depth))
            .collect(),
        width,
        depth,
    )
}

#[cfg(test)]
mod day15_tests {
    use aoc2021::str_as_vec;

    use super::*;

    #[test]
    fn test_wrapped_parse() {
        let (map, w, d) = wrapped_parse(&str_as_vec(
            "12
34",
        ));

        assert_eq!(10, w);
        assert_eq!(10, d);
        assert_eq!(100, map.len());
    }

    #[test]
    fn test_parse() {
        let (map, w, d) = parse(&str_as_vec(
            "01
23",
        ));
        assert_eq!(2, w);
        assert_eq!(2, d);
        assert_eq!(
            vec![
                vec![Edge { node: 1, cost: 1 }, Edge { node: 2, cost: 2 }],
                vec![Edge { node: 0, cost: 0 }, Edge { node: 3, cost: 3 }],
                vec![Edge { node: 3, cost: 3 }, Edge { node: 0, cost: 0 }],
                vec![Edge { node: 2, cost: 2 }, Edge { node: 1, cost: 1 }],
            ],
            map
        );
    }

    #[test]
    fn test_adj() {
        let adj_map = vec![
            vec![Edge { node: 1, cost: 1 }, Edge { node: 2, cost: 2 }],
            vec![Edge { node: 0, cost: 0 }, Edge { node: 3, cost: 3 }],
            vec![Edge { node: 3, cost: 3 }, Edge { node: 0, cost: 0 }],
            vec![Edge { node: 2, cost: 2 }, Edge { node: 1, cost: 1 }],
        ];

        assert_eq!(adj_map[0], make_adj(&[0, 1, 2, 3], 0, 2, 2));
        assert_eq!(adj_map[1], make_adj(&[0, 1, 2, 3], 1, 2, 2));
        assert_eq!(adj_map[2], make_adj(&[0, 1, 2, 3], 2, 2, 2));
        assert_eq!(adj_map[3], make_adj(&[0, 1, 2, 3], 3, 2, 2));
    }

    const TEST_DATA: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_case() {
        let (map, width, depth) = parse(&str_as_vec(TEST_DATA));

        assert_eq!(Some(40), shortest_path(&map, 0, width * depth - 1));
    }
}
