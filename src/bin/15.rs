use advent_of_code::{
    char_enum,
    directions::{Direction, DOWN, LEFT, RIGHT, UP},
    grid::{Grid, IntoGrid, Point},
};

advent_of_code::solution!(15);

char_enum! {
    GridObject {
        Empty = '.',
        Robot = '@',
        Wall = '#',
        Box = 'O',
        BoxLeft = '[',
        BoxRight = ']'
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    use GridObject::*;

    let (grid_input, sequence_input) = input.split_once("\n\n").unwrap();

    let mut grid: Grid<GridObject> = grid_input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.try_into().expect("unexpected item in bagging area"))
        })
        .into_grid();

    let sequence = sequence_input
        .chars()
        .filter_map(|c| Direction::from_arrow(c))
        .collect::<Vec<_>>();

    let mut current_position = grid.index_of(Robot).unwrap();
    grid[current_position] = Empty;

    for step in sequence {
        let next_position = current_position + step;

        // oh god how is this going to fuck me in part two
        match grid.get(next_position) {
            Some(Empty) => current_position = next_position,
            Some(Box) => {
                // try to figure out where the box goes
                let mut position_check = next_position + step;
                loop {
                    match grid.get(position_check) {
                        Some(Box) => {
                            position_check += step;
                        }
                        Some(Empty) => {
                            grid[position_check] = Box;
                            grid[next_position] = Empty;
                            current_position = next_position;
                            break;
                        }
                        _ => break,
                    }
                }
            }
            _ => {}
        }
    }

    let answer: i64 = grid
        .enumerate()
        .filter_map(|loc| (grid[loc] == Box).then_some(loc.y + loc.x * 100))
        .sum();

    Some(answer.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    use GridObject::*;

    let (grid_input, sequence_input) = input.split_once("\n\n").unwrap();

    let mut grid = grid_input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| {
                    match c {
                        '#' => [Wall, Wall],
                        '@' => [Robot, Empty],
                        'O' => [BoxLeft, BoxRight],
                        '.' => [Empty, Empty],
                        _ => panic!("unexpected item in bagging area"),
                    }
                    .into_iter()
                })
                .flatten()
        })
        .into_grid();

    let sequence = sequence_input
        .chars()
        .filter_map(|c| Direction::from_arrow(c))
        .collect::<Vec<_>>();

    let mut current_position = grid.index_of(Robot).unwrap();

    const MOVE_BOX: fn(Point, Direction, &mut Grid<GridObject>, bool) -> bool =
        |loc, dir, grid, lookahead| match dir {
            LEFT => {
                let next_position = loc + dir;
                match grid.get(next_position) {
                    Some(Empty) => {
                        if !lookahead {
                            grid[next_position] = BoxLeft;
                            grid[loc] = BoxRight;
                            grid[loc + RIGHT] = Empty;
                        }
                        true
                    }
                    Some(BoxRight) => {
                        if MOVE_BOX(next_position + dir, dir, grid, lookahead) {
                            if !lookahead {
                                grid[loc + dir] = BoxLeft;
                                grid[loc] = BoxRight;
                                grid[loc + RIGHT] = Empty;
                            }
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            RIGHT => {
                let next_position = loc + dir * 2;
                match grid.get(next_position) {
                    Some(Empty) => {
                        if !lookahead {
                            grid[next_position] = BoxRight;
                            grid[loc + dir] = BoxLeft;
                            grid[loc] = Empty;
                        }
                        true
                    }
                    Some(BoxLeft) => {
                        if MOVE_BOX(next_position, dir, grid, lookahead) {
                            if !lookahead {
                                grid[next_position] = BoxRight;
                                grid[loc + dir] = BoxLeft;
                                grid[loc] = Empty;
                            }
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
            UP | DOWN => match (grid.get(loc + dir), grid.get(loc + dir + RIGHT)) {
                (Some(Empty), Some(Empty)) => {
                    if !lookahead {
                        grid[loc + dir] = BoxLeft;
                        grid[loc + dir + RIGHT] = BoxRight;
                        grid[loc] = Empty;
                        grid[loc + RIGHT] = Empty;
                    }
                    true
                }
                (Some(BoxLeft), Some(BoxRight)) => {
                    if MOVE_BOX(loc + dir, dir, grid, lookahead) {
                        if !lookahead {
                            grid[loc + dir] = BoxLeft;
                            grid[loc + dir + RIGHT] = BoxRight;
                            grid[loc] = Empty;
                            grid[loc + RIGHT] = Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
                (Some(BoxRight), Some(Empty)) => {
                    if MOVE_BOX(loc + dir + LEFT, dir, grid, lookahead) {
                        if !lookahead {
                            grid[loc + dir] = BoxLeft;
                            grid[loc + dir + RIGHT] = BoxRight;
                            grid[loc] = Empty;
                            grid[loc + RIGHT] = Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
                (Some(Empty), Some(BoxLeft)) => {
                    if MOVE_BOX(loc + dir + RIGHT, dir, grid, lookahead) {
                        if !lookahead {
                            grid[loc + dir] = BoxLeft;
                            grid[loc + dir + RIGHT] = BoxRight;
                            grid[loc] = Empty;
                            grid[loc + RIGHT] = Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
                (Some(BoxRight), Some(BoxLeft)) => {
                    if MOVE_BOX(loc + dir + LEFT, dir, grid, lookahead)
                        && MOVE_BOX(loc + dir + RIGHT, dir, grid, lookahead)
                    {
                        if !lookahead {
                            grid[loc + dir] = BoxLeft;
                            grid[loc + dir + RIGHT] = BoxRight;
                            grid[loc] = Empty;
                            grid[loc + RIGHT] = Empty;
                        }
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        };

    for step in sequence {
        let next_position = current_position + step;

        match grid.get(next_position) {
            Some(Empty) => {
                grid[current_position] = Empty;
                grid[next_position] = Robot;
                current_position = next_position;
            }
            Some(t @ BoxLeft) | Some(t @ BoxRight) => {
                let leftbox = if *t == BoxLeft {
                    next_position
                } else {
                    next_position + LEFT
                };
                if MOVE_BOX(leftbox, step, &mut grid, true) {
                    MOVE_BOX(leftbox, step, &mut grid, false);
                    grid[current_position] = Empty;
                    grid[next_position] = Robot;
                    current_position = next_position;
                }
            }
            _ => {}
        }
    }

    let answer: i64 = grid
        .enumerate()
        .filter_map(|loc| (grid[loc] == BoxLeft).then_some(loc.y + loc.x * 100))
        .sum();

    Some(answer.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_large() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
    }
}
