use advent_of_code::{directions::Direction, grid::Point};
use regex::Regex;

advent_of_code::solution!(14);

pub fn part_one_sub<const GX: i64, const GY: i64>(input: &str) -> Option<u32> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots = regex.captures_iter(input).map(|x| {
        let (_, [start_x, start_y, v_x, v_y]) = x.extract();
        (
            Point {
                x: start_x.parse().unwrap(),
                y: start_y.parse().unwrap(),
            },
            Direction {
                x: v_x.parse().unwrap(),
                y: v_y.parse().unwrap(),
            },
        )
    });

    let final_robots = robots.map(|(position, velocity)| {
        let final_position = position + velocity * 100;

        (
            (final_position.x % GX + GX) % GX,
            (final_position.y % GY + GY) % GY,
        )
    });

    let quads = final_robots.fold([0, 0, 0, 0], |mut sectors, (x, y)| {
        let gx = GX / 2;
        let gy = GY / 2;

        // honestly i don't see why match shouldn't be able to desugar to this.
        // my assumption is that it can't guarantee exhaustion but i had a _ case so idk
        // alternatively, why not let me do math on my const generics and use those? annoying
        if x < gx && y < gy {
            sectors[0] += 1;
        }
        if x < gx && y > gy {
            sectors[1] += 1;
        }
        if x > gx && y < gy {
            sectors[2] += 1;
        }
        if x > gx && y > gy {
            sectors[3] += 1;
        }

        sectors
    });

    let answer = quads.into_iter().product();

    Some(answer)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_sub::<101, 103>(input)
}

pub fn part_two_sub<const GX: i64, const GY: i64>(input: &str) -> Option<u32> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut robots: Vec<_> = regex
        .captures_iter(input)
        .map(|x| {
            let (_, [start_x, start_y, v_x, v_y]) = x.extract();
            (
                Point {
                    x: start_x.parse().unwrap(),
                    y: start_y.parse().unwrap(),
                },
                Direction {
                    x: v_x.parse().unwrap(),
                    y: v_y.parse().unwrap(),
                },
            )
        })
        .collect();

    for x in 1..100000 {
        for robot in robots.iter_mut() {
            let point = robot.0 + robot.1;
            *robot = (
                Point {
                    x: (point.x % GX + GX) % GX,
                    y: (point.y % GY + GY),
                },
                robot.1,
            );
        }

        // "looks like christmas tree"
        // O(n^2) :(
        // runs very slowly
        let uhh: i64 = robots
            .iter()
            .map(|first| {
                robots.iter().fold(0, |c, second| {
                    c + (first.0.x - second.0.x).abs() + (first.0.y - second.0.y).abs()
                })
            })
            .sum();
        if uhh < 10_000_000 {
            // is this a good threshold??? probably not????
            // nvm this works lmao
            return Some(x);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_sub::<101, 103>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_sub::<11, 7>(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
