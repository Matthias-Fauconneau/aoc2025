#![feature(iter_next_chunk)]
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let points = input.lines().map(|line| line.split(',').map(|x| x.parse::<f32>().unwrap()).next_chunk().unwrap()).collect::<Vec<_>>();
    let mut circuits : Vec<Vec<usize>> = vec![];
    for _ in 0..10 {
        fn sq(x: f32) -> f32 { x*x }
        let (a,b,_) = points.iter().enumerate().map(|(a,_)| points.iter().enumerate().filter_map({let ref c = circuits; move |(b,_)| (a!=b && !c.iter().any(|c| c.contains(&a) && c.contains(&b))).then(|| [a,b])})).flatten().map(|[a,b]| {
            let [[ax,ay,az],[bx,by,bz]] = [points[a], points[b]];
            (a,b,sq(ax-bx)+sq(ay-by)+sq(az-bz))
        }).min_by(|(_,_,d0),(_,_,d1)| f32::total_cmp(d0, d1)).unwrap();
        println!("{a} {b}");
        let circuit_a = circuits.iter_mut().position(|p| p.contains(&a));
        let circuit_b = circuits.iter_mut().position(|p| p.contains(&b));
        match [circuit_a, circuit_b] {
            [Some(circuit_a),Some(circuit_b)] => {
                println!("{:?} + {:?}", circuits[circuit_a], circuits[circuit_b]);
                let b = circuits.remove(circuit_b).into_iter();
                circuits[circuit_a].extend(b);
            }
            [None,Some(circuit)] => { println!("{:?} + {a}", circuits[circuit]); circuits[circuit].push(a); },
            [Some(circuit),None] => { println!("{:?} + {b}",circuits[circuit]); circuits[circuit].push(b); },
            [None, None] => { println!("new"); circuits.push(vec![a,b]); },
        }
        println!("{circuits:?}");
    }
    circuits.sort_by_key(|c:&Vec<_>| c.len());
    Some(circuits.iter().rev().take(3).map(|c| c.len()).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
