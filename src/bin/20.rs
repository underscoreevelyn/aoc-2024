use advent_of_code::{
    char_enum,
    directions::CARDINAL_DIRECTIONS,
    grid::{Grid, IntoGrid},
};

advent_of_code::solution!(20);

char_enum! {
    GridObject {
        Empty = '.',
        Start = 'S',
        End = 'E',
        Wall = '#'
    }
}

// the funny part about this is that it runs in the exact same amount of time for each part lol
// i'm sure there's some clever things i could do but it runs fast enough that idc
pub fn count_shortcuts<const SAVE: i64, const GLITCH: i64>(input: &str) -> Option<u32> {
    use GridObject::*;

    let grid: Grid<GridObject> = input
        .lines()
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().map(|c| c.try_into().unwrap()))
        .into_grid();

    let mut pos = grid.index_of(Start).unwrap();
    let mut dir = CARDINAL_DIRECTIONS
        .into_iter()
        .find(|x| grid.get(pos + x) == Some(&Empty))
        .unwrap();

    let mut points = vec![pos];

    while grid[pos] != End {
        let (next_pos, next_dir) = [dir, dir.clockwise(), dir.counter_clockwise()]
            .into_iter()
            .find_map(|x| {
                let next_pos = pos + x;
                (grid[next_pos] != Wall).then_some((next_pos, x))
            })
            .expect("wait, are we stuck?");
        points.push(next_pos);
        pos = next_pos;
        dir = next_dir;
    }

    let answer: usize = (0..points.len())
        .map(|i| {
            ((i + 1)..points.len())
                .filter(|j| {
                    let first = points[i];
                    let second = points[*j];

                    let distance = first.manhattan_distance(&second);
                    let skipped_time: i64 =
                        i64::try_from(*j).unwrap() - i64::try_from(i).unwrap() - distance;

                    distance <= GLITCH && skipped_time >= SAVE
                })
                .count()
        })
        .sum();

    Some(answer.try_into().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    count_shortcuts::<100, 2>(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    count_shortcuts::<100, 20>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = count_shortcuts::<1, 2>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result =
            count_shortcuts::<50, 20>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
