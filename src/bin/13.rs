use advent_of_code::{directions::Direction, grid::Point};
use regex::Regex;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    button_a: Direction,
    button_b: Direction,
    target: Point,
}

fn capture_to_machine(value: [&str; 6], part_two: bool) -> Machine {
    let factor = if part_two { 10_000_000_000_000 } else { 0 };
    Machine {
        button_a: Direction {
            x: value[0].parse().unwrap(),
            y: value[1].parse().unwrap(),
        },
        button_b: Direction {
            x: value[2].parse().unwrap(),
            y: value[3].parse().unwrap(),
        },
        target: Point {
            x: value[4].parse::<i64>().unwrap() + factor,
            y: value[5].parse::<i64>().unwrap() + factor,
        },
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut coins_spent = 0;

    for Machine {
        button_a,
        button_b,
        target,
    } in regex
        .captures_iter(input)
        .map(|x| capture_to_machine(x.extract().1, false))
    {
        let top = button_a.x * target.y - button_a.y * target.x;
        let bottom = button_a.x * button_b.y - button_a.y * button_b.x;
        let y = top / bottom;
        let x_top = target.x - button_b.x * y;
        let x_bottom = button_a.x;
        let x = x_top / x_bottom;

        if top % bottom == 0 && x_top % x_bottom == 0 {
            assert_eq!(Point { x: 0, y: 0 } + button_a * x + button_b * y, target);
            coins_spent += y + 3 * x;
        }
    }

    Some(coins_spent as u64)
}

// so i spent about ten billion years freaking out that this exact code wasn't working because
// unbeknownst to me (dumbass) casting an i64 to a u32 fucking overflows it. i cannot express
// how mad i am. from now on i commit to never asing a integral type again. i have learned
// the error of my ways. i will try_into().unwrap(). i'm so fucking tired
pub fn part_two(input: &str) -> Option<u64> {
    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let coins_spent: i64 = regex
        .captures_iter(input)
        .map(|x| capture_to_machine(x.extract().1, true))
        .map(
            |Machine {
                 button_a,
                 button_b,
                 target,
             }| {
                let top = button_a.x * target.y - button_a.y * target.x;
                let bottom = button_a.x * button_b.y - button_a.y * button_b.x;
                let y = top / bottom;
                let x_top = target.x - button_b.x * y;
                let x_bottom = button_a.x;
                let x = x_top / x_bottom;

                if (top % bottom == 0) && (x_top % x_bottom == 0) {
                    y + 3 * x
                } else {
                    0
                }
            },
        )
        .sum();

    Some(coins_spent as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
