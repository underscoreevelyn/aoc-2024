use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let bits: Vec<_> = line.split(' ').collect();
        left.push(bits[0].parse().unwrap());
        right.push(bits[3].parse().unwrap());
    }

    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right.into_iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let bits: Vec<_> = line.split(' ').collect();
        left.push(bits[0].parse().unwrap());
        *right.entry(bits[3].parse().unwrap()).or_insert(0) += 1;
    }

    Some(
        left.into_iter()
            .map(|x| x * right.get(&x).unwrap_or(&0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
