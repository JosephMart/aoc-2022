#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut max_val = 0;
    let mut current_val = 0;
    for l in input.lines() {
        if l.eq("") {
            max_val = std::cmp::max(max_val, current_val);
            current_val = 0;
            continue;
        }
        current_val += l.parse::<i32>().unwrap();
    }
    return std::cmp::max(max_val, current_val);
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut vals = Vec::new();
    let mut current_val = 0;

    for l in input.lines() {
        if l.eq("") {
            vals.push(current_val);
            current_val = 0;
            continue;
        }
        current_val += l.parse::<i32>().unwrap();
    }
    vals.push(current_val);
    vals.sort_by(|a, b| b.cmp(a));

    return vals[0] + vals[1] + vals[2];
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static SAMPLE_VALS: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn sample1() {
        assert_eq!(part1(SAMPLE_VALS), 24000);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(SAMPLE_VALS), 45000);
    }
}
