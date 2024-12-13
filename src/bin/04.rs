use advent_of_code::directions::{Direction, DIRECTIONS};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut xmases = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 'X' {
                for Direction { x: dx, y: dy } in DIRECTIONS {
                    let x = x as i64;
                    let y = y as i64;

                    if x + dx * 3 < 0
                        || y + dy * 3 < 0
                        || x + dx * 3 >= grid.len() as i64
                        || y + dy * 3 >= grid[0].len() as i64
                    {
                        continue;
                    }

                    if grid[(x + dx) as usize][(y + dy) as usize] == 'M'
                        && grid[(x + dx * 2) as usize][(y + dy * 2) as usize] == 'A'
                        && grid[(x + dx * 3) as usize][(y + dy * 3) as usize] == 'S'
                    {
                        xmases += 1;
                    }
                }
            }
        }
    }

    Some(xmases)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut mases = 0;
    for x in 1..(grid.len() - 1) {
        for y in 1..(grid[0].len() - 1) {
            if grid[x][y] == 'A' {
                if ((grid[x - 1][y - 1] == 'M' && grid[x + 1][y + 1] == 'S')
                    || (grid[x - 1][y - 1] == 'S' && grid[x + 1][y + 1] == 'M'))
                    && ((grid[x - 1][y + 1] == 'S' && grid[x + 1][y - 1] == 'M')
                        || (grid[x - 1][y + 1] == 'M' && grid[x + 1][y - 1] == 'S'))
                {
                    mases += 1;
                }
            }
        }
    }

    Some(mases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
