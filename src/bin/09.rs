advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let corners = input.lines().map(|corner| { let (x,y) = corner.split_once(',').unwrap(); [x,y].map(|s| s.parse::<isize>().unwrap()) } ).collect::<Vec<_>>();
    corners.iter().map(|[ax,ay]| corners.iter().map(move |[bx,by]| ((bx-ax).abs()+1)*((by-ay).abs()+1))).flatten().max()
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
