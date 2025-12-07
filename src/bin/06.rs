advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let (operations, operands) = lines.split_last().unwrap();
    let operands = operands.into_iter().map(|op| op.split_whitespace().map(|op| op.parse::<u64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    Some(operations.split_whitespace().enumerate().map(|(j, op)| {
        let column = operands.iter().map(|op| op[j]);
        match op {
            "+" => column.sum::<u64>(),
            "*" => column.product::<u64>(),
            _ => unreachable!()
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let (operations, operands) = lines.split_last().unwrap();
    let width = operands.iter().map(|op| op.len()).max().unwrap();
    let columns = (0..width).map(|j| String::from_utf8(operands.iter().map(|op| *op.as_bytes().get(j).unwrap_or(&b' ')).collect::<Vec<_>>()).unwrap());
    let columns = columns.map(|op| op.trim().parse::<u64>().ok()).collect::<Vec<_>>(); // ( Some+ None )+
    let mut operands = vec![vec![]];
    for op in columns { // ( Some(op)+ None )+ => [[op]]
        if op.is_none() { operands.push(vec![]); }
        else { operands.last_mut().unwrap().push(op.unwrap()) }
    }
    Some(operations.split_whitespace().zip(operands).map(|(op, ops)| {
        match op {
            "+" => ops.into_iter().sum::<u64>(),
            "*" => ops.into_iter().product::<u64>(),
            _ => unreachable!()
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
