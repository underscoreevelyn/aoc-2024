advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = input
        .lines()
        .map(|x| x.split(' ').map(|x| x.parse::<i32>().unwrap()).collect());

    let answer = puzzle
        .map(|x: Vec<_>| x.windows(2).map(|x| x[0] - x[1]).collect())
        .filter(|x: &Vec<_>| {
            x.iter().all(|&x| x <= -1 && x >= -3) || x.iter().all(|&x| x >= 1 && x <= 3)
        })
        .count();

    Some(answer as u32)
}

/// this feels really awkward, i feel like there's a more
/// elegant way to express these relationships
pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = input
        .lines()
        .map(|x| x.split(' ').map(|x| x.parse::<i32>().unwrap()).collect());

    let answer = puzzle
        .filter(|line: &Vec<_>| {
            let diffs: Vec<_> = line.windows(2).map(|x| x[0] - x[1]).collect();
            let negative_version: Vec<_> = diffs
                .iter()
                .enumerate()
                .filter(|(_i, &x)| !(x <= -1 && x >= -3))
                .collect();

            if negative_version.len() == 0 {
                return true;
            }

            if negative_version.len() == 1 {
                let (discontinuity, diff) = negative_version[0];

                if discontinuity == 0 || discontinuity == diffs.len() - 1 {
                    return true;
                }

                let left_diff = diff + diffs[discontinuity - 1];
                if left_diff <= -1 && left_diff >= -3 {
                    return true;
                }
                let right_diff = diff + diffs[discontinuity + 1];
                if right_diff <= -1 && right_diff >= -3 {
                    return true;
                }
            }

            if negative_version.len() == 2 {
                let left = negative_version[0];
                let right = negative_version[1];

                if left.0 + 1 == right.0 {
                    let diff = left.1 + right.1;
                    if diff <= -1 && diff >= -3 {
                        return true;
                    }
                }
            }

            let positive_version: Vec<_> = diffs
                .iter()
                .enumerate()
                .filter(|(_i, &x)| !(x >= 1 && x <= 3))
                .collect();

            if positive_version.len() == 1 {
                let (discontinuity, diff) = positive_version[0];

                if discontinuity == 0 || discontinuity == diffs.len() - 1 {
                    return true;
                }

                let left_diff = diff + diffs[discontinuity - 1];
                if left_diff >= 1 && left_diff <= 3 {
                    return true;
                }
                let right_diff = diff + diffs[discontinuity + 1];
                if right_diff >= 1 && right_diff <= 3 {
                    return true;
                }
            }

            if positive_version.len() == 2 {
                let left = positive_version[0];
                let right = positive_version[1];

                if left.0 + 1 == right.0 {
                    let diff = left.1 + right.1;
                    if diff >= 1 && diff <= 3 {
                        return true;
                    }
                }
            }

            positive_version.len() == 0
        })
        .count();

    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
