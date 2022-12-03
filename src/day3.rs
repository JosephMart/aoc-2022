use std::collections::HashSet;

fn get_priority(c: &u8) -> i32 {
    let v = i32::from(*c);
    match v {
        65..=90 => v - 64 + 26,
        97..=122 => v - 96,
        _ => unreachable!(),
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let mut m: HashSet<&u8> = HashSet::new();
    let mut result = 0;

    for l in input.lines() {
        m.clear();
        let mid = l.len() / 2;
        let collection_0 = &l.as_bytes()[..mid];
        let collection_1 = &l.as_bytes()[mid..];

        for c in collection_0 {
            m.insert(c);
        }

        for c in collection_1 {
            if m.contains(c) {
                result += get_priority(c);
                break;
            }
        }
    }

    result
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut badge_set0 = HashSet::new();
    let mut badge_set1 = HashSet::new();

    let mut result = 0;

    for (i, l) in input.lines().enumerate() {
        let k = i % 3;
        let badge = l.as_bytes();

        match k {
            // how index with ownership...?
            0 => {
                for c in badge {
                    badge_set0.insert(c);
                }
            }
            1 => {
                for c in badge {
                    badge_set1.insert(c);
                }
            }
            2 => {
                for c in badge {
                    if badge_set0.contains(c) && badge_set1.contains(c) {
                        result += get_priority(&c);
                        break;
                    }
                }
                badge_set0.clear();
                badge_set1.clear();
            }
            _ => unreachable!(),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_VALS: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn sample1() {
        assert_eq!(part1("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(part1("afZa"), 1);
        assert_eq!(part1("zfZz"), 26);
        assert_eq!(part1("AfZA"), 27);
        assert_eq!(part1("ZfZz"), 52);
    }

    #[test]
    fn sample2() {
        assert_eq!(
            part2(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"
            ),
            18
        );

        assert_eq!(
            part2(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            52
        );

        assert_eq!(part2(SAMPLE_VALS), 70);
    }
}
