advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (fresh, available) = input.split_once("\n\n").unwrap();
    let fresh = fresh.lines().map(|range| { let (start, end) = range.split_once('-').unwrap(); start.parse::<usize>().unwrap() ..= end.parse::<usize>().unwrap() }).collect::<Vec<_>>();
    Some(available.lines().filter(|id| { let id = id.parse::<usize>().unwrap(); fresh.iter().any(|range| range.contains(&id)) }).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (fresh, _) = input.split_once("\n\n").unwrap();
    #[derive(PartialEq)] enum Tag { Start, Stop } use Tag::*;
    let mut fresh = fresh.lines().map(|range| { let (start, end) = range.split_once('-').unwrap(); [(Start,start.parse::<usize>().unwrap()),(Stop,end.parse::<usize>().unwrap())].into_iter() }).flatten().collect::<Vec<_>>();
    fresh.sort_by_key(|&(_,id)| id);
    let mut first_fresh = None;
    let mut overlap = 0;
    let mut count = 0;
    for (tag, id) in fresh {
        if tag == Start {
            if overlap == 0 { first_fresh = Some(id); }
            overlap += 1;
        }
        if tag == Stop {
            assert!(overlap > 0);
            overlap -= 1;
            if overlap == 0 { count += id+1-first_fresh.take().unwrap(); }
        }
    }
    Some(count)
    //Some((0..=*fresh.iter().map(|range| range.end()).max().unwrap()).filter(|id| fresh.iter().any(|range| range.contains(&id)) ).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
