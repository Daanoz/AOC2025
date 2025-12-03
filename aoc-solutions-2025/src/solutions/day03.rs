use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 3)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input_lines()
            .into_iter()
            .map(|line| find_highest(line, 2))
            .sum::<usize>()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        puzzle
            .get_input_lines()
            .into_iter()
            .map(|line| find_highest(line, 12))
            .sum::<usize>()
            .into()
    }
}

fn find_highest(line: &str, size: usize) -> usize {
    let digits = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut batteries = vec![];
    let mut start_index = 0;
    while batteries.len() < size {
        let end_index = digits.len() - (size - batteries.len()) + 1;
        let (ind, val) = digits[start_index..end_index].iter().enumerate().fold(
            (0, 0),
            |(cur_ind, cur_val), (ind, val)| {
                if *val > cur_val {
                    (ind, *val)
                } else {
                    (cur_ind, cur_val)
                }
            },
        );
        batteries.push(val);
        start_index = start_index + ind + 1;
    }
    batteries.iter().enumerate().fold(0, |acc, (ind, val)| {
        acc + val * 10_usize.pow((size - ind - 1) as u32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"987654321111111
811111111111119
234234234234278
818181911112111"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day.part1(&get_puzzle());
        assert_eq!(result, 357.into());
    }

    #[test]
    fn part2() {
        let result = Day.part2(&get_puzzle());
        assert_eq!(result, 3121910778619_usize.into());
    }
}
