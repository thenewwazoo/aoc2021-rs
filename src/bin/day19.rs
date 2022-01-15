use aoc2021::lines_as_vec;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let lines = lines_as_vec("input/day19.txt");
    let mut dists: Vec<(i32, i32, i32)> = parse_lines(&lines)
        .values()
        .flat_map(|v| dist_map(&v))
        .flatten()
        .collect();
    dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
    dists.dedup();
    dists.retain(|&v| v != (0, 0, 0));
    dists.len()
}

fn parse_lines(lines: &[String]) -> HashMap<usize, Vec<(i32, i32, i32)>> {
    let mut scanner_id = 0;
    let mut output = HashMap::new();
    for line in lines {
        if line == "" {
            continue;
        }
        if line.starts_with("--- ") {
            scanner_id = line.split(' ').nth(2).unwrap().parse::<usize>().unwrap();
            continue;
        }
        let coords: Vec<i32> = line.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        output
            .entry(scanner_id)
            .or_insert_with(Vec::new)
            .push((coords[0], coords[1], coords[2]));
    }
    output
}

fn dist_map(beacons: &[(i32, i32, i32)]) -> Vec<Vec<(i32, i32, i32)>> {
    // fn dist_map(beacons: &[(i32, i32, i32)]) -> Vec<Vec<f64>> {
    beacons
        .iter()
        .map(|b| {
            beacons
                .iter()
                .map(|x| {
                    ((b.0 - x.0), (b.1 - x.1), (b.2 - x.2))
                    //f64::sqrt(((b.0 - x.0).pow(2) + (b.1 - x.1).pow(2) + (b.2 - x.2).pow(2)) as f64)
                })
                .collect()
        })
        .collect()
}

//#[cfg(test)]
/*
mod day19_tests {

    use super::*;

    use aoc2021::str_as_vec;
    use std::cmp::Ordering;

    #[test]
    fn test_dists() {
        let lines = str_as_vec(SIMPLE_TEST_DATA);
        let map = parse_lines(&lines);

        let mut dists: Vec<Vec<(i32, i32, i32)>> = map
            .values()
            .inspect(|l| println!("{:?}", l))
            .map(|v| {
                let mut edges: Vec<(i32, i32, i32)> = dist_map(&v).into_iter().flatten().collect();
                edges.sort_by(|a, b| a.partial_cmp(b).unwrap());
                edges.dedup();
                edges.retain(|&v| v != (0, 0, 0));
                println!("dedups {:?}", edges.len());
                edges
            })
            .collect();

        let mut dupes = Vec::new();
        let mut flat_seen: Vec<(i32, i32, i32)> =
            dists.iter().map(|d| d.clone()).flatten().collect();
        flat_seen.sort_by(|a, b| a.partial_cmp(b).unwrap());
        &flat_seen.windows(2).for_each(|w| {
            if matches!(w[0].partial_cmp(&w[1]), Some(Ordering::Equal)) {
                dupes.push(w[0]);
            }
        });
        dupes.dedup();

        let counts: Vec<usize> = dists
            .clone()
            .into_iter()
            .map(|mut edges| {
                println!("edges starts as {:?}", edges);
                let l = edges.len();
                edges.retain(|e| !&dupes.contains(e));
                println!("l {} but edges {:?}", l, edges);
                edges.len() / l
            })
            .collect();

        println!("dists {:?}", dists);
        println!("flat_seen {:?}", flat_seen);
        println!("dupes {:?}", dupes);
        println!("counts {:?}", counts);

        assert!(false);
    }

    #[test]
    fn test_parse() {
        let lines = str_as_vec("--- scanner 0 ---\n404,-588,-901\n\n--- scanner 10 ---\n1,-2,3");
        let map = parse_lines(&lines);

        assert_eq!(
            HashMap::from([(0, vec![(404, -588, -901)]), (10, vec![(1, -2, 3)])]),
            map
        );
    }

    const TEST_DATA_2D: &str = "--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0";

    const SIMPLE_TEST_DATA: &str = "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 1 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 2 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 3 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 4 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";

    const TEST_DATA: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
}
*/
