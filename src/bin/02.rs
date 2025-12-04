advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for range in input.split(',') {
        let (min, max) = range.split_once('-').unwrap();
        let [min, max] = [min, max].map(|s| s.parse::<u64>().unwrap());
        for i in min..=max {
            let id = i.to_string();
            if id[0..id.len()/2] == id[id.len()/2..id.len()] { println!("{i}"); sum += i; }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for range in input.split(',') {
        let (min, max) = range.split_once('-').unwrap();
        let [min, max] = [min, max].map(|s| s.parse::<u64>().unwrap());
        for i in min..=max {
            let id = i.to_string();
            for len in 1..=id.len()/2 {
                if id.len()%len != 0 { continue; }
                if (1..id.len()/len).all(|t| id[t*len..t*len+len] == id[0..len]) { println!("{i}"); sum += i; break; }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
