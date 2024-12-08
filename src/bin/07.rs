advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations = input.lines().map(|line| {
        // could maybe use a regex, not sure
        let portions: Vec<_> = line.split(": ").collect();
        (
            portions[0].parse::<u64>().unwrap(),
            portions[1]
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    // brute force it
    let answer = equations
        .filter_map(|(answer, args)| {
            let mut possibilities = vec![args[0]];

            for n in args.into_iter().skip(1) {
                for i in 0..possibilities.len() {
                    possibilities.push(n * possibilities[i]);
                    possibilities[i] += n;
                }
            }

            possibilities.contains(&answer).then_some(answer)
        })
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = input.lines().map(|line| {
        // could maybe use a regex, not sure
        let portions: Vec<_> = line.split(": ").collect();
        (
            portions[0].parse::<u64>().unwrap(),
            portions[1]
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>(),
        )
    });

    // brute force it, but again
    let answer = equations
        .filter_map(|(answer, args)| {
            let mut possibilities = vec![args[0]];

            for n in args.into_iter().skip(1) {
                for i in 0..possibilities.len() {
                    possibilities.push(n * possibilities[i]);
                    possibilities.push(possibilities[i] * 10u64.pow(n.ilog10() + 1) + n);
                    possibilities[i] += n;
                }
            }

            possibilities.contains(&answer).then_some(answer)
        })
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
