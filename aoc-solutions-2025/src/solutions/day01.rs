use std::ops::Range;

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 1)]
#[derive(Default)]
pub struct Day;

const DIAL_RANGE: Range<i32> = 0..100;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let mut pos = 50;
        let mut zero_count = 0;
        puzzle.get_input_lines().iter().for_each(|line| {
            let move_by = match line.split_at(1) {
                ("L", v) => -(v.parse::<i32>().unwrap()),
                ("R", v) => v.parse::<i32>().unwrap(),
                _ => panic!("Invalid direction"),
            };
            pos = (pos + move_by).rem_euclid(DIAL_RANGE.end);
            if pos == 0 {
                zero_count += 1;
            }
        });
        zero_count.into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let mut pos = 50;
        let mut zero_count = 0;
        puzzle.get_input_lines().iter().for_each(|line| {   
            let move_by = match line.split_at(1) {
                ("L", v) => -(v.parse::<i32>().unwrap()),
                ("R", v) => v.parse::<i32>().unwrap(),
                _ => panic!("Invalid direction"),
            };
            let start_pos = pos;
            let new_pos = start_pos + move_by;
            pos = new_pos.rem_euclid(DIAL_RANGE.end);
            let mut number_of_zeros = 0;
            if pos == 0 {
                number_of_zeros += 1;
            }
            if !DIAL_RANGE.contains(&new_pos) {
                let mut delta_count = (new_pos.abs_diff(pos) as i32) / DIAL_RANGE.end;
                // trim edges
                if (start_pos == 0 && move_by < 0) || (move_by > 0 && pos == 0) {
                    delta_count -= 1;
                }
                number_of_zeros += delta_count;
            }
            zero_count += number_of_zeros;
        });
        zero_count.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#,
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
        assert_eq!(result, 6.into());
    }
}
