use std::collections::HashMap;

advent_of_code::solution!(11);

// so i do have to wonder if there's some optimization here, since this runs in, uh, fucking forever
// i wonder if i can do dynamic programming...
pub fn solve_slow(input: &str, iters: usize) -> u64 {
    let mut numbers: Vec<u64> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    for _ in 0..iters {
        // manually iterate >:(
        let mut i = 0;
        while i < numbers.len() {
            if numbers[i] == 0 {
                numbers[i] = 1;
            } else if numbers[i].ilog10() % 2 == 0 {
                numbers[i] *= 2024;
            } else {
                let digits = numbers[i].ilog10() / 2 + 1;
                let factor = 10u64.pow(digits);
                numbers.insert(i, numbers[i] % factor);
                i += 1;
                numbers[i] /= factor;
            }
            i += 1;
        }
    }

    numbers.len() as u64
}

// yes, i could use dynamic programming
// this is even faster than part one on the bigger input and i find that very funny
pub fn solve_fast(input: &str, iters: usize) -> u64 {
    let mut memo = HashMap::new();

    const RECURSE: fn(u64, usize, &mut HashMap<(u64, usize), u64>) -> u64 = |n, depth, memo| {
        if depth == 0 {
            return 1;
        }

        if n == 0 {
            return RECURSE(1, depth - 1, memo);
        }

        if let Some(ans) = memo.get(&(n, depth)) {
            return *ans;
        }

        let log = n.ilog10();

        if log % 2 == 1 {
            let factor = 10u64.pow(log / 2 + 1);
            let ans = RECURSE(n / factor, depth - 1, memo) + RECURSE(n % factor, depth - 1, memo);
            memo.insert((n, depth), ans);
            ans
        } else {
            let ans = RECURSE(n * 2024, depth - 1, memo);
            memo.insert((n, depth), ans);
            ans
        }
    };

    input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .map(|n| RECURSE(n, iters, &mut memo))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_fast(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_fast(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_fast() {
        let input = advent_of_code::template::read_file("examples", DAY);
        for i in 0..15 {
            assert_eq!(solve_slow(&input, i), solve_fast(&input, i));
        }
    }
}
