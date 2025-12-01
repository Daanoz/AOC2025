use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 1)]
#[derive(Default)]
pub struct Day;

impl PuzzleSolution for Day {
    fn part1(&self, _puzzle: &Puzzle) -> Answer {
        "".into()
    }

    fn part2(&self, _puzzle: &Puzzle) -> Answer {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#""#,
        )
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, "".into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, "".into());
    }
}
