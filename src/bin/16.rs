use std::collections::{HashMap, HashSet};

use advent_of_code::{
    char_enum,
    directions::{Direction, DOWN, LEFT, RIGHT, UP},
    djikstra::{djikstra, djikstra_paths},
    grid::{Grid, IntoGrid, Point},
};

advent_of_code::solution!(16);

char_enum! {
    GridObject {
        Wall = '#',
        Empty = '.',
        Start = 'S',
        End = 'E'
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    position: Point,
    direction: Direction,
}

pub fn part_one(input: &str) -> Option<u32> {
    use GridObject::*;

    let grid: Grid<GridObject> = input
        .lines()
        .map(|x| x.chars().map(|c| c.try_into().unwrap()))
        .into_grid();

    let next_states = |State {
                           position,
                           direction,
                       }| {
        let mut v = vec![
            (
                State {
                    position,
                    direction: direction.clockwise(),
                },
                1000,
            ),
            (
                State {
                    position,
                    direction: direction.counter_clockwise(),
                },
                1000,
            ),
        ];
        let forward_position = grid.get(position + direction);

        if forward_position == Some(&Empty) || forward_position == Some(&End) {
            v.push((
                State {
                    position: position + direction,
                    direction,
                },
                1,
            ));
        }
        v
    };

    let start_position = grid.index_of(Start).unwrap();
    let end_position = grid.index_of(End).unwrap();

    let start = State {
        position: start_position,
        direction: RIGHT,
    };

    let end = [
        State {
            position: end_position,
            direction: UP,
        },
        State {
            position: end_position,
            direction: LEFT,
        },
        State {
            position: end_position,
            direction: DOWN,
        },
        State {
            position: end_position,
            direction: RIGHT,
        },
    ];

    let answer = djikstra(start, &end, next_states).unwrap();

    Some(answer.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    use GridObject::*;

    let grid: Grid<GridObject> = input
        .lines()
        .map(|x| x.chars().map(|c| c.try_into().unwrap()))
        .into_grid();

    let next_states = |State {
                           position,
                           direction,
                       }| {
        let mut v = vec![
            (
                State {
                    position,
                    direction: direction.clockwise(),
                },
                1000,
            ),
            (
                State {
                    position,
                    direction: direction.counter_clockwise(),
                },
                1000,
            ),
        ];
        let forward_position = grid.get(position + direction);

        if forward_position == Some(&Empty) || forward_position == Some(&End) {
            v.push((
                State {
                    position: position + direction,
                    direction,
                },
                1,
            ));
        }
        v
    };

    let start_position = grid.index_of(Start).unwrap();
    let end_position = grid.index_of(End).unwrap();

    let start = State {
        position: start_position,
        direction: RIGHT,
    };

    let states = djikstra_paths(start, next_states);

    let end = [
        State {
            position: end_position,
            direction: UP,
        },
        State {
            position: end_position,
            direction: LEFT,
        },
        State {
            position: end_position,
            direction: DOWN,
        },
        State {
            position: end_position,
            direction: RIGHT,
        },
    ];

    let min_score = end
        .iter()
        .map(|x| states.get(x).and_then(|x| Some(x.0)).unwrap_or(usize::MAX))
        .min();

    let valid_ends: Vec<_> = end
        .into_iter()
        .filter(|x| states.get(x).and_then(|x| Some(x.0)) == min_score)
        .collect();

    const RECURSE: fn(&[State], &HashMap<State, (usize, Vec<State>)>, &mut HashSet<Point>) =
        |cur_states, states, set| {
            for state in cur_states {
                set.insert(state.position);
                if let Some(next_states) = states.get(state) {
                    RECURSE(&next_states.1, states, set);
                }
            }
        };

    let mut spots = HashSet::new();

    RECURSE(&valid_ends, &states, &mut spots);

    let answer = spots.len();

    Some(answer.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
