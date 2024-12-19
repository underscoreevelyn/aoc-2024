use std::collections::HashSet;

use advent_of_code::{
    directions::*,
    grid::{Grid, IntoGrid, Point},
};

advent_of_code::solution!(12);

// i feel like the easiest way to do this is with flood fill, right?
pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().map(|x| x.chars()).into_grid();

    // so uh, flood fill
    // (area, perimeter)
    let mut visited = HashSet::new();
    const RECURSE: fn(Point, char, &mut HashSet<Point>, &Grid<char>) -> (usize, usize) =
        |loc, c, visited, grid| {
            if visited.contains(&loc) {
                return (0, 0);
            }

            visited.insert(loc);

            if grid.get(loc) == Some(&c) {
                let (area, perimeter) = CARDINAL_DIRECTIONS
                    .into_iter()
                    .map(|dir| (loc + dir, grid.get(loc + dir)))
                    .fold((0, 0), |(area, perimeter), (new_loc, new_c)| {
                        if new_c == Some(&c) {
                            let (r_area, r_perimeter) = RECURSE(new_loc, c, visited, grid);
                            (area + r_area, perimeter + r_perimeter)
                        } else {
                            (area, perimeter + 1)
                        }
                    });
                (area + 1, perimeter)
            } else {
                (0, 0)
            }
        };

    let answer = grid
        .enumerate()
        .map(|loc| RECURSE(loc, grid[loc], &mut visited, &grid))
        .fold(0, |total, (a, b)| total + a * b);

    Some(answer as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|x| x.chars()).into_grid();

    // so uh, flood fill
    // (area, perimeter)
    let mut visited = HashSet::new();
    const RECURSE: fn(
        Point,
        char,
        &mut HashSet<Point>,
        &Grid<char>,
        &mut Vec<(Point, Direction)>,
    ) -> usize = |loc, c, visited, grid, fences| {
        if visited.contains(&loc) {
            return 0;
        }

        visited.insert(loc);

        if grid.get(loc) == Some(&c) {
            let mut area = 0;
            for dir in CARDINAL_DIRECTIONS {
                let new_loc = loc + dir;
                if grid.get(new_loc) == Some(&c) {
                    area += RECURSE(new_loc, c, visited, grid, fences);
                } else {
                    // there is a fence between loc and new_loc.
                    fences.push((loc, dir));
                }
            }

            area + 1
        } else {
            0
        }
    };

    let answer = grid
        .enumerate()
        .map(|loc| {
            let mut v = vec![];
            let area = RECURSE(loc, grid[loc], &mut visited, &grid, &mut v);
            let fences = dedup_fences(v);
            (area, fences)
        })
        .fold(0, |total, (a, b)| total + a * b.len());

    Some(answer as u32)
}

// this is probably not particularly efficient
// nevermind it actually doesn't run that poorly ;-;
fn dedup_fences(mut fences: Vec<(Point, Direction)>) -> Vec<(Point, Direction)> {
    let mut i = 0;
    while i < fences.len() {
        let (start, dir) = fences[i];
        let dir1 = dir.clockwise();
        let dir2 = dir.counter_clockwise();
        let mut cur = start + dir1;
        while let Some(pos) = fences.iter().position(|x| x == &(cur, dir)) {
            fences.remove(pos);
            cur = cur + dir1;
        }
        cur = start + dir2;
        while let Some(pos) = fences.iter().position(|x| x == &(cur, dir)) {
            fences.remove(pos);
            cur = cur + dir2;
        }
        i += 1;
    }

    fences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
