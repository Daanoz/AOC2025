use std::num::ParseIntError;

use aoc_core::{aoc_puzzle, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 2)]
#[derive(Default)]
pub struct Day;

type Num = u64;

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        get_ranges(puzzle)
            .map(|result| {
                result.map(|input_range| {
                    let mut sum = 0;
                    let (mut start, mut end) = (input_range.0, input_range.1);
                    let range = start..=end;

                    let digits_start = digit_count(start);
                    let digits_end = digit_count(end);
                    if digits_start % 2 == 1 && digits_end % 2 == 1 {
                        // No valid sequences possible for this range
                        return 0;
                    }
                    let mut digits = digits_start;
                    if digits_start % 2 == 1 {
                        // Move to next even digit count, uneven digits cannot be split evenly
                        start = 10u64.pow(digits_start);
                        digits += 1;
                    }
                    if digits_end % 2 == 1 {
                        // Move to previous even digit count, uneven digits cannot be split evenly
                        end = 10u64.pow(digits_end - 1) - 1;
                    }

                    // Determine range for left side of the number
                    let power = 10u64.pow(digits / 2);
                    let left_side_start = start / power;
                    let left_side_end = end / power;

                    // Check each numbers by mirroring left side to right side
                    for l in left_side_start..=left_side_end {
                        let val = l + (l * power);
                        if range.contains(&val) {
                            sum += val;
                        }
                    }

                    sum
                })
            })
            .sum::<Result<u64, ParseIntError>>()
            .expect("Valid input")
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        get_ranges(puzzle)
            .map(|result| {
                result.map(|input_range| {
                    let mut sum = 0;
                    let (start, end) = (input_range.0, input_range.1);
                    let range = start..=end;

                    let mut hits = vec![];

                    let digits_start = digit_count(start);
                    let digits_end = digit_count(end);
                    // We assume ranges to not differ by more than one digit count
                    let verify_ranges = if digits_start == digits_end {
                        vec![(start, end, digits_start)]
                    } else {
                        vec![
                            (start, 10u64.pow(digits_start) - 1, digits_start),
                            (10u64.pow(digits_end - 1), end, digits_end),
                        ]
                    };

                    for (start, end, digits) in verify_ranges.clone() {
                        for pattern_width in 1..=(digits / 2) {
                            if digits % pattern_width != 0 {
                                // Not divisible, skip
                                continue;
                            }
                            let match_start = start / 10u64.pow(digits - pattern_width);
                            let match_end = end / 10u64.pow(digits - pattern_width);
                            for pattern in match_start..=match_end {
                                // Recreate product_id by repeating with power of pattern_width
                                let repeated_pattern = (0..(digits / pattern_width))
                                    .fold(0u64, |acc, _| {
                                        acc * 10u64.pow(pattern_width) + pattern
                                    });
                                if range.contains(&repeated_pattern)
                                    && !hits.contains(&repeated_pattern)
                                {
                                    sum += repeated_pattern;
                                    hits.push(repeated_pattern);
                                }
                            }
                        }
                    }

                    sum
                })
            })
            .sum::<Result<u64, ParseIntError>>()
            .expect("Valid input")
            .into()
    }
}

fn get_ranges(input: &Puzzle) -> impl Iterator<Item = Result<(Num, Num), ParseIntError>> + use<'_> {
    input.input_as_str().split(",").map(|r| {
        let (start, end) = r.trim().split_once('-').unwrap();
        Ok((start.parse()?, end.parse()?))
    })
}

fn digit_count(n: Num) -> u32 {
    if n == 0 {
        return 1;
    }
    n.ilog10() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(
            r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#,
        )
    }

    #[test]
    fn part1() {
        let result = Day.part1(&get_puzzle());
        assert_eq!(result, 1227775554.into());
    }

    #[test]
    fn part2() {
        let result = Day.part2(&get_puzzle());
        assert_eq!(result, 4174379265_usize.into());
    }
}
