advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    let array = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let mut accessible = 0;
    for i in 0..height {
        for j in 0..width {
            if array[i][j] != b'@' { print!("."); continue; }
            let mut adjacent = 0;
            for di in -1..=1 { for dj in -1..=1 {
                if di == 0 && dj == 0 { continue; }
                let [i, j] = [i as isize+di, j as isize+dj];
                if i < 0 || i >= height as isize || j < 0 || j >= width as isize { continue; }
                if array[i as usize][j as usize] == b'@' { adjacent += 1; }
            }}
            if adjacent < 4 { print!("x"); accessible += 1; } else { print!("@"); }
        }
        println!("");
    }
    Some(accessible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    let mut array = input.lines().map(|line| line.as_bytes().to_owned()).collect::<Vec<_>>();
    let mut total = 0;
    loop {
        let mut removed = 0;
        for i in 0..height {
            for j in 0..width {
                if array[i][j] != b'@' { continue; }
                let mut adjacent = 0;
                for di in -1..=1 { for dj in -1..=1 {
                    if di == 0 && dj == 0 { continue; }
                    let [i, j] = [i as isize+di, j as isize+dj];
                    if i < 0 || i >= height as isize || j < 0 || j >= width as isize { continue; }
                    if array[i as usize][j as usize] == b'@' { adjacent += 1; }
                }}
                if adjacent < 4 { array[i][j] = b'x'; removed += 1; }
            }
        }
        println!("{removed}");
        if removed == 0 { break; }
        total += removed;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
