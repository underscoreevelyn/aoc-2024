advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let mut locks: Vec<[usize; 5]> = Vec::new();
    let mut keys: Vec<[usize; 5]> = Vec::new();

    for thing in input.split("\n\n") {
        let grid: Vec<Vec<_>> = thing.lines().map(|x| x.chars().collect()).collect();

        let mut pins = [0; 5];
        for i in 0..pins.len() {
            pins[i] = grid.iter().filter(|x| x[i] == '#').count();
        }

        if grid[0][0] == '#' {
            locks.push(pins);
        } else {
            keys.push(pins);
        }
    }

    let answer: usize = locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    lock.iter()
                        .zip(key.iter())
                        .all(|(lock, key)| lock + key <= 7)
                })
                .count()
        })
        .sum();

    Some(answer.try_into().unwrap())
}

pub fn part_two(_input: &str) -> Option<u32> {
    println!("no part two on chrimas :3");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
