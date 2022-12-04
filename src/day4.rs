use std::str::FromStr;

#[derive(Debug)]

pub struct Assignment {
    l: (u8, u8),
    r: (u8, u8),
}

impl FromStr for Assignment {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(",").unwrap();
        let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
        Ok(Assignment {
            l: (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()),
            r: (c.parse::<u8>().unwrap(), d.parse::<u8>().unwrap()),
        })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .into_iter()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(assignments: &[Assignment]) -> usize {
    assignments
        .into_iter()
        .filter(|a| (a.l.0 >= a.r.0 && a.l.1 <= a.r.1) || (a.r.0 >= a.l.0 && a.r.1 <= a.l.1))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(assignments: &[Assignment]) -> usize {
    assignments
        .into_iter()
        .filter(|a| {
            (a.l.0 >= a.r.0 && a.l.0 <= a.r.1)
                || (a.l.1 >= a.r.0 && a.l.1 <= a.r.1)
                || (a.r.0 >= a.l.0 && a.r.0 <= a.l.1)
                || (a.r.1 >= a.l.0 && a.r.1 <= a.l.1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_VALS: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_VALS)), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_VALS)), 4);
    }
}
