advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| {
        let line = line.as_bytes();
        let mut max = 0;
        for i in 0..line.len()-1 { if line[i] > line[max] { max = i; } }
        let mut max2 = max+1;
        for i in max+1..line.len() { if line[i] > line[max2] { max2 = i; } }
        let joltage = (line[max]-b'0') as u64 * 10 + (line[max2]-b'0') as u64;
        println!("{joltage}");
        joltage
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| {
        let bank = line.as_bytes();
        let mut max = 0;
        let mut joltage = 0;
        for n in (1..=12).rev() {
            for i in max..bank.len()-n { if bank[i] > bank[max] { max = i; } }
            joltage *= 10;
            joltage += (bank[max]-b'0') as u64;
            max += 1;
        }
        println!("-{line}");
        println!("+{joltage}");
        joltage
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
