use advent_of_code::{
    directions::{self, Direction},
    grid::{Grid, IntoGrid},
};

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

    let mut grid: Grid<_> = input
        .lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '#' => Obstructed,
                '.' => Empty,
                '^' => Visited,
                _ => panic!("Uhhhhhh what the hell is this?"),
            })
        })
        .into_grid();

    let mut current_loc = grid.index_of(Visited).expect("no starting point?");
    let mut current_dir = directions::UP;

    let mut seen = 1;
    'outer: loop {
        if grid[current_loc] == Empty {
            grid[current_loc] = Visited;
            seen += 1;
        }

        for _ in 0..4 {
            let next_loc = current_loc + current_dir;
            match grid.get(next_loc) {
                None => break 'outer,
                Some(Obstructed) => current_dir = current_dir.clockwise(),
                _ => {
                    current_loc = next_loc;
                    break;
                }
            }
        }
    }

    Some(seen)
}

// too tired to think of a clever solution, used brute force and it worked
// nice 17s runtime on day 6, love that
pub fn part_two(input: &str) -> Option<u32> {
    use Space::*;

    let grid: Grid<_> = input
        .lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '#' => Obstructed,
                '.' => Empty,
                '^' => Visited,
                _ => panic!("Uhhhhhh what the hell is this?"),
            })
        })
        .into_grid();

    let start_loc = grid.index_of(Visited).expect("No starting point?");

    let mut options = 0;

    'bigouter: for p in grid.enumerate() {
        let mut grid = grid.clone();

        if grid[p] == Obstructed || grid[p] == Visited {
            continue;
        }
        grid[p] = Obstructed;

        let mut current_loc = start_loc;
        let mut current_dir = directions::UP;

        'outer: loop {
            if grid[current_loc] == Empty || grid[current_loc] == Visited {
                let mut dirs = [false; 4];
                dirs[dir_to_index(current_dir)] = true;
                grid[current_loc] = Visited2(dirs);
            } else if let Visited2(mut dirs) = grid[current_loc] {
                if dirs[dir_to_index(current_dir)] {
                    options += 1;
                    continue 'bigouter;
                }
                dirs[dir_to_index(current_dir)] = true;
            }

            for _ in 0..4 {
                let next_loc = current_loc + current_dir;
                match grid.get(next_loc) {
                    None => break 'outer,
                    Some(Obstructed) => current_dir = current_dir.clockwise(),
                    _ => {
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
