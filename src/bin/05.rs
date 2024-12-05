use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    // ok so the meaning of the rules was really unclear to me
    // ;-;
    let rules: HashSet<(usize, usize)> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let mut it = x.split('|').map(|x| x.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();

    let books: Vec<Vec<usize>> = input
        .lines()
        .skip_while(|x| !x.is_empty())
        .filter(|x| !x.is_empty())
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    // brute force??
    let answer: usize = books
        .iter()
        .filter(|book| {
            (0..book.len())
                .all(|i| ((i + 1)..book.len()).all(|j| !rules.contains(&(book[j], book[i]))))
        })
        .map(|x| x[x.len() / 2])
        .sum();

    Some(answer as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules: HashSet<(usize, usize)> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let mut it = x.split('|').map(|x| x.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();

    let books: Vec<Vec<usize>> = input
        .lines()
        .skip_while(|x| !x.is_empty())
        .filter(|x| !x.is_empty())
        .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    let incorrect_books = books.iter().filter(|book| {
        (0..book.len()).any(|i| ((i + 1)..book.len()).any(|j| rules.contains(&(book[j], book[i]))))
    });

    // so logically there needs to be exactly one correct ordering.
    // which means there's some specific structure to the rules, i think i can just
    let answer: usize = incorrect_books
        .map(|book| {
            let mut v = Vec::with_capacity(book.len());

            // greedily input them in order as far to the right as i can
            // honestly i'm surprised this works? but eh, structure :)
            'outer: for &n in book.into_iter() {
                for i in 0..v.len() {
                    if rules.contains(&(n, v[i])) {
                        v.insert(i, n);
                        continue 'outer;
                    }
                }

                v.push(n);
            }

            v[v.len() / 2]
        })
        .sum();

    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
