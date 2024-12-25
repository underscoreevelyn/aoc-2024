use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashSet, VecDeque},
};

advent_of_code::solution!(24);

#[derive(Debug, PartialEq)]
enum Op {
    Xor,
    Or,
    And,
}

// if this turns into sat i am going to scream
pub fn part_one(input: &str) -> Option<u64> {
    use Op::*;

    let (initial, relations) = input.split_once("\n\n").unwrap();

    let mut values: BTreeMap<_, _> = initial
        .lines()
        .filter_map(|x| {
            x.split_once(": ")
                .map(|(v, n)| (Reverse(v), n.parse::<u64>().unwrap()))
        })
        .collect();

    let mut rules: VecDeque<_> = relations
        .lines()
        .map(|x| {
            let parts: Vec<_> = x.split(' ').collect();
            let op = match parts[1] {
                "XOR" => Xor,
                "OR" => Or,
                "AND" => And,
                _ => panic!("unexpected item in bagging area"),
            };

            (op, parts[0], parts[2], parts[4])
        })
        .collect();

    while let Some((op, lhs, rhs, out)) = rules.pop_front() {
        if values.contains_key(&Reverse(lhs)) && values.contains_key(&Reverse(rhs)) {
            let lhs = values.get(&Reverse(lhs)).unwrap();
            let rhs = values.get(&Reverse(rhs)).unwrap();
            values.insert(
                Reverse(out),
                match op {
                    Xor => lhs ^ rhs,
                    Or => lhs | rhs,
                    And => lhs & rhs,
                },
            );
        } else {
            rules.push_back((op, lhs, rhs, out));
        }
    }

    let mut val = 0;
    for (Reverse(key), value) in values {
        if !key.starts_with("z") {
            break;
        }
        val = (val << 1) | value;
    }

    Some(val)
}

// so i could immediately tell this was just a chain of full adders, so i could just check to
// make sure everything lined up
// however i could not for the life of me figure out how to get this shit to work, i ended up
// looking at a solution just to figure out if i was barking up the right tree and theirs
// was literally just mine but with some small differences, so that was cool
// at least i had the right idea :sob:
pub fn part_two(input: &str) -> Option<String> {
    use Op::*;

    let (_initial, relations) = input.split_once("\n\n").unwrap();

    let rules: Vec<_> = relations
        .lines()
        .map(|x| {
            let parts: Vec<_> = x.split(' ').collect();
            let op = match parts[1] {
                "XOR" => Xor,
                "OR" => Or,
                "AND" => And,
                _ => panic!("unexpected item in bagging area"),
            };

            (op, parts[0].min(parts[2]), parts[2].max(parts[0]), parts[4])
        })
        .collect();

    let find_by_input_op = |op: Op, l: Option<&str>, r: Option<&str>| {
        l.and_then(|l| {
            r.and_then(|r| {
                rules
                    .iter()
                    .find(|x| x.0 == op && x.1 == l.min(r) && x.2 == r.max(l))
                    .map(|x| x.3)
            })
        })
    };
    let find_by_output = |out: &str| rules.iter().find(|x| x.3 == out);
    let find_by_single_input = |op: Op, input: Option<&str>| {
        input.and_then(|input| {
            rules
                .iter()
                .find(|x| x.0 == op && (x.1 == input || x.2 == input))
                .map(|x| x.3)
        })
    };

    let xs = (1..=44).map(|i| format!("x{i:02}")).collect::<Vec<_>>();
    let ys = (1..=44).map(|i| format!("y{i:02}")).collect::<Vec<_>>();
    let zs = (1..=44).map(|i| format!("z{i:02}")).collect::<Vec<_>>();

    let mut carries = vec![find_by_input_op(And, Some("x00"), Some("y00")).unwrap()];
    let mut what_the_fuck = HashSet::new();

    // so i have NO idea what's going on with the nullables here, i was running into problems
    // with unwraps panicking so i just started keeping everything around as options and for some
    // reason it works now ;-;
    for i in 1..=44 {
        let x = &xs[i - 1];
        let y = &ys[i - 1];
        let z = &zs[i - 1];

        let carry = carries[i - 1];
        let xory = find_by_input_op(Xor, Some(x), Some(y));
        let out = find_by_input_op(Xor, xory, Some(carry));

        let (out, carry, xory) = if let Some(out) = out {
            (out, carry, xory)
        // xory ^ carry should be z, because out doesn't exist, either xory or carry is swapped
        } else {
            let (_, lhs, rhs, _) = find_by_output(z).unwrap();
            let (carry, xory) = if xory == Some(*lhs) || xory == Some(*rhs) {
                what_the_fuck.insert(carry);
                let carry = if Some(*lhs) == xory { rhs } else { lhs };
                what_the_fuck.insert(carry);
                (*carry, xory)
            } else {
                what_the_fuck.insert(xory.unwrap());
                let xory = if *lhs == carry { rhs } else { lhs };
                what_the_fuck.insert(xory);
                (carry, Some(*xory))
            };
            (
                find_by_input_op(Xor, xory, Some(carry)).unwrap(),
                carry,
                xory,
            )
        };

        // out should be z, these are switched
        if out != z {
            what_the_fuck.insert(out);
            what_the_fuck.insert(z);
        }

        let xandy = find_by_input_op(And, Some(x), Some(y));
        let and_carry = find_by_input_op(And, xory, Some(carry));
        let or_carry = find_by_input_op(Or, xandy, and_carry);

        let carry_out = if let Some(or_carry) = or_carry {
            or_carry
        } else {
            // if or_carry can't be found, either xandy or and_carry is swapped
            // this is the part that i couldn't really figure out, figuring out which one to swap
            // with them was really unintuitive to me
            // oh well
            let help = find_by_single_input(Or, xandy);
            let me = find_by_single_input(Or, and_carry);
            let (xandy, and_carry) = if me.is_none() {
                what_the_fuck.insert(and_carry.unwrap());
                let (_, a, b, _) = find_by_output(help.unwrap()).unwrap();
                let and_carry = if Some(*a) == xandy { b } else { a };
                what_the_fuck.insert(and_carry);
                (xandy, Some(*and_carry))
            } else if help.is_none() {
                what_the_fuck.insert(xandy.unwrap());
                let (_, a, b, _) = find_by_output(me.unwrap()).unwrap();
                let xandy = if Some(*a) == and_carry { b } else { a };
                what_the_fuck.insert(xandy);
                (Some(*xandy), and_carry)
            } else {
                (xandy, and_carry) // ?
            };
            find_by_input_op(Or, xandy, and_carry).unwrap()
        };

        carries.push(carry_out);
    }

    let mut aaaaaaa: Vec<_> = what_the_fuck.into_iter().collect();
    aaaaaaa.sort();

    Some(aaaaaaa.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }
}
