advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.as_bytes().split(|&b| b == b'\n').collect::<Vec<_>>();
    let mut beam = vec![lines[0].iter().position(|&b| b == b'S').unwrap() as isize];
    let mut split = 0;
    for line in &lines[2..] {
        let mut next = vec![];
        for j in beam {
            if j<0 || j >= line.len() as isize { continue; }
            match line[j as usize] {
                b'.' => if !next.contains(&j) { next.push(j) },
                b'^' => {
                    if !next.contains(&(j-1)) { next.push(j-1) }
                    if !next.contains(&(j+1)) { next.push(j+1) } split+=1; },
                _ => unreachable!()
            }
        }
        beam = next;
        for (j, c) in line.iter().enumerate() {
            if beam.contains(&(j as isize)) { print!("|"); } else { print!("{}", char::from(*c)) }
        }
        println!();
    }
    Some(split)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.as_bytes().split(|&b| b == b'\n').collect::<Vec<_>>();
    let W = lines[0].len();
    let mut beam = vec![0; W];
    beam[lines[0].iter().position(|&b| b == b'S').unwrap()] = 1;
    let mut timelines = 1;
    for line in &lines[2..] {
        let mut next = vec![0; W];
        for (j, count) in beam.iter().enumerate() {
            match line[j as usize] {
                b'.' => next[j] += count,
                b'^' => { if j>0 { next[j-1] += count; } if j<W-1 { next[j+1] += count; } timelines+=count; }
                _ => unreachable!()
            }
        }
        beam = next;
        for (j, c) in line.iter().enumerate() {
            if beam[j] > 0 { print!("|"); } else { print!("{}", char::from(*c)) }
        }
        println!();
    }
    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
