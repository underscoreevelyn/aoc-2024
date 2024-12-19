use std::collections::HashSet;

use advent_of_code::{
    directions::CARDINAL_DIRECTIONS,
    djikstra::djikstra,
    grid::{Grid, Point},
};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    part_one_sub::<71, 1024>(input)
}

pub fn part_one_sub<const DIM: i64, const SIM: usize>(input: &str) -> Option<u32> {
    let corruptions: HashSet<_> = input
        .lines()
        .take(SIM)
        .map(|x| {
            let mut bits = x.split(',');
            Point {
                x: bits.next().unwrap().parse().unwrap(),
                y: bits.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let next_states = |point| {
        CARDINAL_DIRECTIONS
            .into_iter()
            .filter_map(|x| {
                let p: Point = point + x;

                (p.x >= 0 && p.y >= 0 && p.x < DIM && p.y < DIM && !corruptions.contains(&p))
                    .then_some((p, 1))
            })
            .collect()
    };

    let distance = djikstra(
        Point { x: 0, y: 0 },
        &[Point {
            x: DIM - 1,
            y: DIM - 1,
        }],
        next_states,
    )
    .unwrap();

    Some(distance.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_sub::<71>(input)
}

#[derive(Clone, PartialEq)]
enum GridObject {
    Empty,
    Blocked,
}

// i'm pretty sure the correct way to do this is to keep the trail and only redo the search when a
// corruption actually lands on your path; however this runs in 30 ish seconds and i'm too lazy for
// that
pub fn part_two_sub<const DIM: i64>(input: &str) -> Option<String> {
    use GridObject::*;

    let mut grid = Grid::new((DIM.try_into().unwrap(), DIM.try_into().unwrap()), Empty);

    let next_states = |grid: Grid<GridObject>| {
        move |point| {
            CARDINAL_DIRECTIONS
                .into_iter()
                .filter_map(|x| {
                    let p: Point = point + x;

                    (grid.get(p) == Some(&Empty)).then_some((p, 1))
                })
                .collect()
        }
    };

    for corruption in input.lines().map(|x| {
        let mut bits = x.split(',');
        Point {
            x: bits.next().unwrap().parse().unwrap(),
            y: bits.next().unwrap().parse().unwrap(),
        }
    }) {
        grid[corruption] = Blocked;
        if djikstra(
            Point { x: 0, y: 0 },
            &[Point {
                x: DIM - 1,
                y: DIM - 1,
            }],
            next_states(grid.clone()),
        ) == None
        {
            return Some(format!("{},{}", corruption.x, corruption.y));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_sub::<7, 12>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_sub::<7>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
