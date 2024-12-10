advent_of_code::solution!(9);

#[derive(Clone, Debug, PartialEq, Eq)]
enum FilesystemObject {
    File(u128),
    Empty,
}

pub fn part_one(input: &str) -> Option<u128> {
    use FilesystemObject::*;

    let files: Vec<_> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .enumerate()
        .map(|(i, c)| {
            let block = if i % 2 == 0 {
                File(i as u128 / 2)
            } else {
                Empty
            };

            vec![block; c.to_digit(10).unwrap() as usize]
        })
        .flatten()
        .collect();

    let mut answer = 0;
    let mut back = files.len() - 1;

    for (i, block) in files.iter().enumerate() {
        if i > back {
            break;
        }

        while files[back] == Empty {
            back -= 1;
        }

        match block {
            File(id) => {
                answer += id * i as u128;
            }
            Empty => {
                let File(id) = files[back] else {
                    panic!("see earlier while")
                };

                answer += id * i as u128;
                back -= 1;
            }
        }
    }

    Some(answer)
}

#[derive(Clone, Debug, PartialEq)]
enum FilesystemSpan {
    File { size: u128, id: u128 },
    Empty { size: u128 },
}

// this takes even longer ;-; avoiding reallocating the vec did not help
pub fn part_two_alt(input: &str) -> Option<u128> {
    use FilesystemObject::*;

    let mut files: Vec<_> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .enumerate()
        .map(|(i, c)| {
            let block = if i % 2 == 0 {
                File(i as u128 / 2)
            } else {
                Empty
            };

            vec![block; c.to_digit(10).unwrap() as usize]
        })
        .flatten()
        .collect();

    let mut i = files.len() - 1;
    while i > 0 {
        match files[i] {
            Empty => i -= 1,
            File(id) => {
                let Some(size) = files[0..=i].iter().rev().position(|x| *x != File(id)) else {
                    i -= 1;
                    continue;
                };
                let base = i - size + 1;

                if let Some(j) = (0..i).find(|&i| files[i..(i + size)].iter().all(|x| *x == Empty))
                {
                    for block in files[j..(j + size)].iter_mut() {
                        *block = File(id);
                    }
                    for block in files[base..(base + size)].iter_mut() {
                        *block = Empty;
                    }
                }
                i -= size;
            }
        }
    }

    Some(
        files
            .into_iter()
            .enumerate()
            .map(|(i, x)| if let File(id) = x { i as u128 * id } else { 0 })
            .sum(),
    )
}

// gonna parse this one differently actually
pub fn part_two(input: &str) -> Option<u128> {
    use FilesystemSpan::*;

    let mut files: Vec<_> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                File {
                    size: c.to_digit(10).unwrap().into(),
                    id: i as u128 / 2,
                }
            } else {
                Empty {
                    size: c.to_digit(10).unwrap().into(),
                }
            }
        })
        .collect();

    let mut i = files.len() - 1;

    // this is surprisingly slow, and i genuinely have no idea why
    // i assume it's because i'm inserting into the array like a moron
    while i > 0 {
        match &files[i] {
            file @ File {
                size: file_size, ..
            } => {
                let file = file.clone();
                let file_size = file_size.clone();
                match files.iter_mut().enumerate().find(|(_, block)| {
                    **block == file
                        || if let Empty { size } = block {
                            *size >= file_size
                        } else {
                            false
                        }
                }) {
                    Some((index, Empty { ref mut size })) => {
                        *size -= file_size;
                        files.insert(index, file);
                        i += 1;
                        files[i] = Empty { size: file_size };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        i -= 1;
    }

    let answer = files
        .into_iter()
        .fold((0, 0), |(sum, current_index), block| match block {
            File { size, id } => (
                sum + id * size * (size + 2 * current_index - 1) / 2,
                current_index + size,
            ),
            Empty { size } => (sum, current_index + size),
        })
        .0;

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
