// ROCK  A <-> X: 1pt
// PAPER B <-> Y : 2pt
// SCISSORS  C <-> Z : 3pt

pub struct Round(String, String);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    let mut vals = Vec::new();

    for l in input.lines() {
        let v: Vec<&str> = l.split(" ").collect();
        vals.push(Round(v[0].to_string(), v[1].to_string()));
    }
    return vals;
}

fn get_beating_val(a: &str) -> &str {
    return match a {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _ => unreachable!(),
    };
}

fn get_losing_val(a: &str) -> &str {
    return match a {
        "A" => "C",
        "B" => "A",
        "C" => "B",
        _ => unreachable!(),
    };
}

fn get_points_won(a: &str, b: &str) -> i128 {
    if b.eq(a) {
        return 3;
    }

    if b.eq(get_beating_val(a)) {
        return 6;
    }

    if b.eq(get_losing_val(a)) {
        return 0;
    }

    unreachable!();
}

// 1 for Rock, 2 for Paper, and 3 for Scissors
fn get_points_from_choice(b: &str) -> i128 {
    return match b {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => unreachable!(),
    };
}

#[aoc(day2, part1)]
pub fn part1(input: &[Round]) -> i128 {
    let mut val = 0;

    for v in input {
        let b = match v.1.as_str() {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => unreachable!(),
        };
        let from_win = get_points_won(&v.0, b);
        let from_choice = get_points_from_choice(b);
        val += from_win + from_choice;
    }

    return val;
}

#[aoc(day2, part2)]
pub fn part2(input: &[Round]) -> i128 {
    let mut val = 0;

    for v in input {
        let b = match v.1.as_str() {
            "X" => get_losing_val(&v.0),
            "Y" => &v.0,
            "Z" => get_beating_val(&v.0),
            _ => unreachable!(),
        };
        let from_choice = get_points_from_choice(b);
        let from_win = get_points_won(&v.0, b);
        let x = from_choice + from_win;
        val += x;
    }

    return val;
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_VALS: &str = "A Y
B X
C Z";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator("A Y")), 8);
        assert_eq!(part1(&input_generator("B X")), 1);
        assert_eq!(part1(&input_generator(SAMPLE_VALS)), 15);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator("A Y")), 4);
        assert_eq!(part2(&input_generator("B X")), 1);
        assert_eq!(part2(&input_generator("C Z")), 7);
        assert_eq!(part2(&input_generator(SAMPLE_VALS)), 12);
    }
}
