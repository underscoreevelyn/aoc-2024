use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

// suspiciously easy part one ;-;
pub fn part_one(input: &str) -> Option<u32> {
    let graph = input.lines().fold(HashMap::new(), |mut map, connection| {
        let Some(connection) = connection.split_once('-') else {
            return map;
        };
        map.entry(connection.0)
            .or_insert(Vec::new())
            .push(connection.1);
        map.entry(connection.1)
            .or_insert(Vec::new())
            .push(connection.0);
        map
    });

    let mut triples = 0;
    let mut checked = HashSet::new();
    for (first, seconds) in &graph {
        if !first.starts_with("t") {
            continue;
        }
        checked.insert(first);
        for second in seconds {
            if checked.contains(second) {
                continue;
            }
            let Some(thirds) = graph.get(second) else {
                continue;
            };
            for third in thirds {
                if checked.contains(third) {
                    continue;
                }
                let Some(firsts) = graph.get(third) else {
                    continue;
                };
                if firsts.contains(first) {
                    triples += 1;
                }
            }
        }
    }

    Some(triples / 2)
}

// i literally accidentally solved this
// i looked it up and apparently this problem is np complete so i thought "eh fuck it i'll just
// make a brute forcer and let it run while i work on implementing an actual algorithm" and somehow
// it runs in 5 seconds lmao
pub fn part_two(input: &str) -> Option<String> {
    let graph = input.lines().fold(HashMap::new(), |mut map, connection| {
        let Some(connection) = connection.split_once('-') else {
            return map;
        };
        map.entry(connection.0)
            .or_insert(HashSet::new())
            .insert(connection.1);
        map.entry(connection.1)
            .or_insert(HashSet::new())
            .insert(connection.0);
        map
    });

    const RECURSE: for<'a> fn(
        &'a str,
        HashSet<&'a str>,
        &HashMap<&'a str, HashSet<&'a str>>,
    ) -> HashSet<&'a str> = |cur_state, posibilities, graph| {
        if posibilities.len() == 0 {
            HashSet::from([cur_state])
        } else {
            let mut seen = HashSet::new();
            posibilities
                .iter()
                .map(|next| {
                    let ret = RECURSE(
                        next,
                        posibilities
                            .intersection(graph.get(next).unwrap())
                            .filter(|x| !seen.contains(x))
                            .map(|x| *x)
                            .collect(),
                        graph,
                    );
                    seen.insert(next);
                    ret
                })
                .max_by(|a, b| a.len().cmp(&b.len()))
                .map(|mut x| {
                    x.insert(cur_state);
                    x
                })
                .unwrap()
        }
    };

    let max_set = graph
        .iter()
        .map(|(start, connections)| RECURSE(start, connections.clone(), &graph))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    let mut oops: Vec<_> = max_set.into_iter().collect();
    oops.sort();
    let answer = oops.join(",");

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
