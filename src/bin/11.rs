advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = input.lines().map(|line| {
        let (device, outputs) = line.split_once(':').unwrap();
        let outputs = outputs.trim().split(' ').collect::<Vec<_>>();
        (device, outputs)
    }).collect::<Vec<_>>();
    fn walk(graph: &[(&str, Vec<&str>)], node: &str) -> usize {
        let mut out = 0;
        for &output in &graph.iter().find(|&&(device,_)| device==node).expect(node).1 {
            if output == "out" { out+=1; }
            else { out += walk(graph, output); }
        }
       out
    }
    Some(walk(&graph, "you"))
}

pub fn part_two(input: &str) -> Option<usize> {
    let graph = input.lines().map(|line| {
        let (device, outputs) = line.split_once(':').unwrap();
        let outputs = outputs.trim().split(' ').collect::<Vec<_>>();
        (device, outputs)
    }).collect::<Vec<_>>();
    fn walk(graph: &[(&str, Vec<&str>)], node: &str, [mut dac, mut fft]: [bool; 2]) -> usize {
        if node == "dac" { dac = true; }
        if node == "fft" { fft = true; }
        let mut out = 0;
        for &output in &graph.iter().find(|&&(device,_)| device==node).expect(node).1 {
            if output == "out" { if dac && fft { out+=1; } }
            else { out += walk(graph, output, [dac, fft]); }
        }
       out
    }
    Some(walk(&graph, "svr", [false, false]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
