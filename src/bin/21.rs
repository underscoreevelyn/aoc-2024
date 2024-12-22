use std::collections::HashMap;

use advent_of_code::grid::{Grid, Point};

advent_of_code::solution!(21);

const ACTIONS: [char; 5] = ['A', '>', '^', '<', 'v'];

fn generate_sequences(pad: Grid<char>) -> HashMap<char, HashMap<char, Vec<String>>> {
    let mut map = HashMap::new();

    for p0 in pad.enumerate() {
        for p1 in pad.enumerate() {
            let a = pad[p0];
            let b = pad[p1];
            let horizontal = p1.x - p0.x;
            let vertical = p1.y - p0.y;
            let horizontal_char = if horizontal > 0 { 'v' } else { '^' };
            let vertical_char = if vertical > 0 { '>' } else { '<' };

            let horizontal_component: String = std::iter::repeat(horizontal_char)
                .take(horizontal.abs() as usize)
                .collect();
            let vertical_component: String = std::iter::repeat(vertical_char)
                .take(vertical.abs() as usize)
                .collect();

            let mut v = vec![];

            if pad[Point { x: p0.x, y: p1.y }] != ' ' {
                v.push(format!("{vertical_component}{horizontal_component}A"));
            }
            if pad[Point { x: p1.x, y: p0.y }] != ' ' {
                v.push(format!("{horizontal_component}{vertical_component}A"));
            }

            map.entry(a).or_insert(HashMap::new()).insert(b, v);
        }
    }
    map
}

fn shortest_sequence(seq: &str, depth: usize) -> usize {
    // fuck it, hardcode it
    let sequences: HashMap<char, HashMap<char, Vec<String>>> =
        generate_sequences(Grid::from_vecs(vec![
            vec![' ', '^', 'A'],
            vec!['<', 'v', '>'],
        ]));

    let mut memo: HashMap<(char, char, usize), usize> = HashMap::new();
    for p0 in ACTIONS {
        for p1 in ACTIONS {
            memo.insert(
                (p0, p1, 0),
                sequences.get(&p0).unwrap().get(&p1).unwrap()[0].len(),
            );
        }
    }

    seq.chars()
        .fold(('A', 0), |(cur, total), next| {
            (
                next,
                total + next_shortest_length(cur, next, depth, &mut memo, &sequences),
            )
        })
        .1
}

fn next_shortest_length(
    start: char,
    end: char,
    depth: usize,
    memo: &mut HashMap<(char, char, usize), usize>,
    sequences: &HashMap<char, HashMap<char, Vec<String>>>,
) -> usize {
    if let Some(v) = memo.get(&(start, end, depth)) {
        return *v;
    }

    let target_sequences = sequences.get(&start).unwrap().get(&end).unwrap();

    let lowest_len = target_sequences
        .iter()
        .map(|target_sequence| {
            target_sequence
                .chars()
                .fold(('A', 0), |(cur, total), next| {
                    (
                        next,
                        total + next_shortest_length(cur, next, depth - 1, memo, sequences),
                    )
                })
                .1
        })
        .min()
        .unwrap();
    memo.insert((start, end, depth), lowest_len);
    lowest_len
}

pub fn prepare_input(
    buttons: &str,
    v: &mut Vec<String>,
    prev: char,
    built: String,
    sequences: &HashMap<char, HashMap<char, Vec<String>>>,
) {
    if let Some(c) = buttons.chars().next() {
        for sequence in sequences.get(&prev).unwrap().get(&c).unwrap() {
            prepare_input(&buttons[1..], v, c, built.clone() + sequence, sequences);
        }
    } else {
        v.push(built);
    }
}

// so wow, i fucking sucked at this one
// i immediately knew that i had to do some dp bullshit but i struggled so hard to write my code in
// a way that actually worked, i kept falling into endless debugging rabbit holes and rewrote my
// code in its entirety like, twice
// i'm sure you can tell my mental state by the variable names in these functions and the fact that
// it's the 22nd as i'm writing this
pub fn part_one(input: &str) -> Option<u64> {
    let answer: usize = input
        .lines()
        .map(|line| {
            let mut idk_anymore = vec![];
            prepare_input(
                line,
                &mut idk_anymore,
                'A',
                String::new(),
                &generate_sequences(Grid::from_vecs(vec![
                    vec!['7', '8', '9'],
                    vec!['4', '5', '6'],
                    vec!['1', '2', '3'],
                    vec![' ', '0', 'A'],
                ])),
            );
            idk_anymore
                .into_iter()
                .map(|augh| shortest_sequence(&augh, 1))
                .min()
                .unwrap()
                * line[0..3].parse::<usize>().unwrap()
        })
        .sum();
    Some(answer.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let answer: usize = input
        .lines()
        .map(|line| {
            let mut idk_anymore = vec![];
            prepare_input(
                line,
                &mut idk_anymore,
                'A',
                String::new(),
                &generate_sequences(Grid::from_vecs(vec![
                    vec!['7', '8', '9'],
                    vec!['4', '5', '6'],
                    vec!['1', '2', '3'],
                    vec![' ', '0', 'A'],
                ])),
            );
            idk_anymore
                .into_iter()
                .map(|augh| shortest_sequence(&augh, 24))
                .min()
                .unwrap()
                * line[0..3].parse::<usize>().unwrap()
        })
        .sum();
    Some(answer.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
