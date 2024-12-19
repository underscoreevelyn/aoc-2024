use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (towels_str, patterns_str) = input.split_once("\n\n").unwrap();
    let towels: Vec<_> = towels_str.split(", ").map(|x| x.as_bytes()).collect();
    let patterns = patterns_str.lines().map(|x| x.as_bytes());

    const DFS: fn(&[u8], &[&[u8]]) -> bool = |goal, patterns| {
        if goal.len() == 0 {
            return true;
        }

        for pattern in patterns {
            if goal.starts_with(pattern) {
                if DFS(&goal[pattern.len()..], patterns) {
                    return true;
                }
            }
        }

        false
    };

    let valid_patterns = patterns.filter(|pattern| DFS(pattern, &towels));

    Some(valid_patterns.count().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels_str, patterns_str) = input.split_once("\n\n").unwrap();
    let towels: Vec<_> = towels_str.split(", ").map(|x| x.as_bytes()).collect();
    let patterns = patterns_str.lines().map(|x| x.as_bytes());

    const DFS: for<'a> fn(&'a [u8], &[&[u8]], &mut HashMap<&'a [u8], u64>) -> u64 =
        |goal, patterns, memo| {
            if goal.len() == 0 {
                return 1;
            }

            if let Some(v) = memo.get(goal) {
                return *v;
            }

            let v = patterns
                .iter()
                .filter_map(|pattern| {
                    goal.starts_with(pattern)
                        .then(|| DFS(&goal[pattern.len()..], patterns, memo))
                })
                .sum();
            memo.insert(goal, v);
            v
        };

    let mut memo = HashMap::new();
    let valid_patterns = patterns.map(|pattern| DFS(pattern, &towels, &mut memo));

    Some(valid_patterns.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
