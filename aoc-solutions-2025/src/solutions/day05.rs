use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 5)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let (fresh_ranges, ingredients) = puzzle.input_as_str().split_once("\n\n").unwrap();
        let fresh_ranges = fresh_ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                (start.parse().unwrap(), end.parse().unwrap())
            })
            .map(|(start, end): (u64, u64)| { start..=end })
            .collect::<Vec<_>>();
        ingredients
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .filter(|ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
            .count()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let (fresh_ranges, _) = puzzle.input_as_str().split_once("\n\n").unwrap();
        let mut fresh_ranges = fresh_ranges
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                (start.parse().unwrap(), end.parse().unwrap())
            })
            .map(|(start, end): (u64, u64)| { start..=end })
            .collect::<Vec<_>>();
        fresh_ranges.sort_by_key(|range| *range.start());
        let mut total = 0;
        let mut current_start = 0;
        let mut current_end = 0;
        for range in fresh_ranges {
            if range.start() > &current_end {
                if current_end != 0 {
                    total += (current_end - current_start) + 1;
                }
                current_start = *range.start();
                current_end = *range.end();
            } else if *range.end() > current_end {
                current_end = *range.end();
            }
        }
        total += (current_end - current_start) + 1;
        total.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 3.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 14.into());
    }
}
