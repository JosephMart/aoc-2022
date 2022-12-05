use std::str::FromStr;

#[derive(Debug)]
pub struct Command {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // move 1 from 3 to 9
        let data: Vec<&str> = s.split_whitespace().collect();
        Ok(Command {
            count: data[1].parse().unwrap(),
            from: data[3].parse::<usize>().unwrap() - 1,
            to: data[5].parse::<usize>().unwrap() - 1,
        })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let (stack_data, commands_data) = input.split_once("\n\n").unwrap();

    for line in stack_data.lines().rev() {
        let bytes = line.as_bytes();
        let length = line.split_whitespace().into_iter().count();

        // Populate on first.
        if stacks.len() == 0 {
            for _ in 0..length {
                stacks.push(Vec::new());
            }
            continue;
        }

        let mut char_i = 1;
        let mut stack_i = 0;

        while char_i < bytes.len() {
            let c = bytes[char_i] as char;
            if c != ' ' {
                stacks[stack_i].push(c);
            }
            char_i += 4;
            stack_i += 1;
        }
    }

    let commands: Vec<Command> = commands_data
        .lines()
        .into_iter()
        .map(|c| c.parse().unwrap())
        .collect();

    (stacks, commands)
}

#[aoc(day5, part1)]
pub fn part1((stacks, commands): &(Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut s = stacks.clone(); // i should read the docs...
    for command in commands {
        for _ in 0..command.count {
            let x = s[command.from].pop().unwrap();
            s[command.to].push(x);
        }
    }

    s.iter().map(|a| a.last().unwrap()).collect()
}

#[aoc(day5, part2)]
pub fn part2((stacks, commands): &(Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut s = stacks.clone(); // i should read the docs...
    for command in commands {
        let mut inbetween = Vec::new();
        for _ in 0..command.count {
            let x = s[command.from].pop().unwrap();
            inbetween.push(x);
        }

        for _ in 0..inbetween.len() {
            s[command.to].push(inbetween.pop().unwrap());
        }
    }

    s.iter().map(|a| a.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_VALS: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_VALS)), "CMZ");
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_VALS)), "MCD");
    }
}
