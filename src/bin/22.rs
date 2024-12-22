use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(22);

const MASK: i128 = 16777216 - 1;

fn next_secret_number(mut input: i128) -> i128 {
    input = (input ^ (input << 6)) & MASK;
    input = (input ^ (input >> 5)) & MASK;
    (input ^ (input << 11)) & MASK
}

pub fn part_one(input: &str) -> Option<i128> {
    let answer = input
        .lines()
        .map(|line| {
            let mut start = line.parse().unwrap();
            for _ in 0..2000 {
                start = next_secret_number(start);
            }
            start
        })
        .sum();

    Some(answer)
}

fn insert_generate_table(mut cur: i128, map: &mut HashMap<(i128, i128, i128, i128), i128>) {
    let mut seen = VecDeque::with_capacity(4);
    let mut seen_indexes = HashSet::new();

    for _ in 0..3 {
        let next = next_secret_number(cur);
        seen.push_back(next % 10 - cur % 10);
        cur = next;
    }

    for _ in 0..1997 {
        let next = next_secret_number(cur);
        seen.push_back(next % 10 - cur % 10);
        let index = (seen[0], seen[1], seen[2], seen[3]);
        seen.pop_front();
        if seen_indexes.insert(index) {
            *map.entry(index).or_default() += next % 10;
        }
        cur = next;
    }
}

// best way to gain time is to abuse the shit out of maps, it is known
pub fn part_two(input: &str) -> Option<i128> {
    let mut stuff = HashMap::new();
    input
        .lines()
        .for_each(|line| insert_generate_table(line.parse().unwrap(), &mut stuff));

    let answer = stuff.values().max().unwrap();

    Some(*answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_gen() {
        let mut secret = 123;
        for (i, expected) in [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ]
        .into_iter()
        .enumerate()
        {
            secret = next_secret_number(secret);
            assert_eq!(secret, expected, "{}", i);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
