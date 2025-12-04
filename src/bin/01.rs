advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut value = 50;
    let mut times = 0;
    for line in input.lines() {
        let (direction, delta) = line.split_at(1);
        let delta: i32 = delta.parse().unwrap();
        match direction {
            "L" => { value -= delta; }
            "R" => { value += delta; }
            _ => unreachable!()
        }
        while value < 0 { value += 100; }
        while value >= 100 { value -= 100; }
        println!("{value}");
        if value == 0 { times += 1; }
    }
    Some(times)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut value = 50;
    let mut times = 0;
    for line in input.lines() {
        let (direction, delta) = line.split_at(1);
        let delta: i32 = delta.parse().unwrap();
        if false {
            match direction {
                "L" => { value -= delta; }
                "R" => { value += delta; }
                _ => unreachable!()
            }
            while value < 0 { value += 100; times += 1; }
            while value >= 100 { value -= 100; times += 1; }
            //-1
        } else {
            match direction {
                "L" => { for _ in 0..delta { value-=1; if value == -1 { value = 99; } if value == 0 { times += 1; } } }
                "R" => { for _ in 0..delta { value+=1; if value == 100 { value = 0; } if value == 0 { times += 1; } } }
                _ => unreachable!()
            }
        }
        println!("{value}");
    }
    Some(times)
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
        assert_eq!(result, Some(6));
    }
}
