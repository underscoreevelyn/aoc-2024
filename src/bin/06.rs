use advent_of_code::directions::{self, Direction};

advent_of_code::solution!(6);

// i wrote this while super tired and boy does it fuckin show

#[derive(PartialEq, Eq, Debug, Clone)]
enum Space {
    Empty,
    Visited,
    Obstructed,
    Visited2([bool; 4]),
}

fn dir_to_index(dir: Direction) -> usize {
    match dir {
        directions::UP => 0,
        directions::RIGHT => 1,
        directions::DOWN => 2,
        directions::LEFT => 3,
        _ => panic!("we don't have diagonals here?"),
    }
}

// feels like a kinda janky way to do this
// i should really write some actual fucking grid code
pub fn part_one(input: &str) -> Option<u32> {
    use Space::*;

    let mut grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Obstructed,
                    '.' => Empty,
                    '^' => Visited,
                    _ => panic!("Uhhhhhh what the hell is this?"),
                })
                .collect()
        })
        .collect();

    let mut current_loc = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, element)| (*element == Visited).then_some((i, j)))
        })
        .expect("No starting point?");
    let mut current_dir = directions::UP;

    let mut seen = 1;
    'outer: loop {
        if grid[current_loc.0][current_loc.1] == Empty {
            grid[current_loc.0][current_loc.1] = Visited;
            seen += 1;
        }

        for _ in 0..4 {
            let next_loc = (
                current_loc.0.wrapping_add(current_dir.0 as usize),
                current_loc.1.wrapping_add(current_dir.1 as usize),
            );
            if next_loc.0 >= grid.len() || next_loc.1 >= grid[0].len() {
                break 'outer;
            } else if grid[next_loc.0][next_loc.1] == Obstructed {
                current_dir = (current_dir.1, -current_dir.0);
            } else {
                current_loc = next_loc;
                break;
            }
        }
    }

    Some(seen)
}

// too tired to think of a clever solution, used brute force and it worked
// nice 17s runtime on day 6, love that
pub fn part_two(input: &str) -> Option<u32> {
    use Space::*;

    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Obstructed,
                    '.' => Empty,
                    '^' => Visited,
                    _ => panic!("Uhhhhhh what the hell is this?"),
                })
                .collect()
        })
        .collect();

    let start_loc = grid
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, element)| (*element == Visited).then_some((i, j)))
        })
        .expect("No starting point?");

    let mut options = 0;

    for i in 0..grid.len() {
        'bigouter: for j in 0..grid[0].len() {
            let mut grid = grid.clone();

            if grid[i][j] == Obstructed || grid[i][j] == Visited {
                continue;
            }
            grid[i][j] = Obstructed;

            let mut current_loc = start_loc;
            let mut current_dir = directions::UP;

            'outer: loop {
                if grid[current_loc.0][current_loc.1] == Empty
                    || grid[current_loc.0][current_loc.1] == Visited
                {
                    let mut dirs = [false; 4];
                    dirs[dir_to_index(current_dir)] = true;
                    grid[current_loc.0][current_loc.1] = Visited2(dirs);
                } else if let Visited2(mut dirs) = grid[current_loc.0][current_loc.1] {
                    if dirs[dir_to_index(current_dir)] {
                        options += 1;
                        continue 'bigouter;
                    }
                    dirs[dir_to_index(current_dir)] = true;
                }

                for _ in 0..4 {
                    let next_loc = (
                        current_loc.0.wrapping_add(current_dir.0 as usize),
                        current_loc.1.wrapping_add(current_dir.1 as usize),
                    );
                    if next_loc.0 >= grid.len() || next_loc.1 >= grid[0].len() {
                        break 'outer;
                    } else if grid[next_loc.0][next_loc.1] == Obstructed {
                        current_dir = (current_dir.1, -current_dir.0);
                    } else {
                        current_loc = next_loc;
                        break;
                    }
                }
            }
        }
    }

    Some(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
