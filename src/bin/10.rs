use std::collections::HashSet;

use advent_of_code::{
    directions::CARDINAL_DIRECTIONS,
    grid::{Grid, IntoGrid, Point},
};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()))
        .into_grid();

    // doing this recursively is easier
    // doing something like dijkstra might be better but i'm gonna guess by not much
    const TRAILS: fn(Point, &Grid<u32>) -> HashSet<Point> =
        |start: Point, grid: &Grid<u32>| -> HashSet<Point> {
            match grid.get(start) {
                Some(9) => HashSet::from_iter(std::iter::once(start)),
                Some(&n) => CARDINAL_DIRECTIONS
                    .iter()
                    .map(|x| start + x)
                    .filter(|x| grid.get(*x) == Some(&(n + 1)))
                    .map(|x| TRAILS(x, grid))
                    .flatten()
                    .collect(),
                None => HashSet::new(),
            }
        };

    let answer: usize = grid
        .enumerate()
        .filter(|x| grid[x] == 0)
        .map(|x| TRAILS(x, &grid).len())
        .sum();

    Some(answer as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()))
        .into_grid();

    // i actually solved this one first by accident because i misunderstood the description lmaooo
    // freest part two of my goddamm life
    const TRAILS: fn(Point, &Grid<u32>) -> u32 = |start: Point, grid: &Grid<u32>| -> u32 {
        match grid.get(start) {
            Some(9) => 1,
            Some(&n) => CARDINAL_DIRECTIONS
                .iter()
                .map(|x| start + x)
                .filter(|x| grid.get(*x) == Some(&(n + 1)))
                .map(|x| TRAILS(x, grid))
                .sum(),
            None => 0,
        }
    };

    let answer = grid
        .enumerate()
        .filter(|x| grid[x] == 0)
        .map(|x| TRAILS(x, &grid))
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
