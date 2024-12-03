advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let answer = regex
        .captures_iter(input)
        .map(|x| {
            let (_, [a, b]) = x.extract();
            a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do()(\(\))|do(n't)(\(\))").unwrap();
    let mut sum = 0;
    let mut active = true;
    for capture in regex.captures_iter(input) {
        let (_, [a, b]) = capture.extract();
        if a == "" {
            active = true;
        } else if a == "n't" {
            active = false;
        } else if active {
            sum += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
