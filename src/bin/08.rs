use std::collections::{HashMap, HashSet};

use advent_of_code::grid::{IntoGrid, Point};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().map(|x| x.chars()).into_grid();

    let nodes = grid.enumerate().fold(HashMap::new(), |mut map, loc| {
        let c = grid[loc];
        if c != '.' {
            map.entry(c).or_insert(vec![]).push(loc);
        }
        map
    });

    let mut antinodes = HashSet::new();
    for locations in nodes.values() {
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let p1 = locations[i];
                let p2 = locations[j];
                let t1 = Point {
                    x: p1.x * 2 - p2.x,
                    y: p1.y * 2 - p2.y,
                };
                let t2 = Point {
                    x: p2.x * 2 - p1.x,
                    y: p2.y * 2 - p1.y,
                };

                if grid.is_inside(t1) {
                    antinodes.insert(t1);
                }
                if grid.is_inside(t2) {
                    antinodes.insert(t2);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|x| x.chars()).into_grid();

    let nodes = grid.enumerate().fold(HashMap::new(), |mut map, loc| {
        let c = grid[loc];
        if c != '.' {
            map.entry(c).or_insert(vec![]).push(loc);
        }
        map
    });

    let mut antinodes = HashSet::new();
    for locations in nodes.values() {
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let p1 = locations[i];
                let p2 = locations[j];
                // this gets kinda weird because of integer math... :(
                // i assume the easy solution will still be fast enough tho sooooo
                // wow it was actually kinda crazy fast ;-;
                for point in grid.enumerate() {
                    if (point.x - p1.x) * (p2.y - p1.y) == (p2.x - p1.x) * (point.y - p1.y) {
                        antinodes.insert(point);
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
