use std::fmt::Display;

use aoc_core::{aoc_puzzle, tools::Grid, Answer, Puzzle, PuzzleSolution};

#[aoc_puzzle(day = 4)]
#[derive(Default)]
pub struct Day;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Roll,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Empty => '.',
            Cell::Roll => '@',
        };
        write!(f, "{}", c)
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '@' => Cell::Roll,
            _ => panic!("Invalid cell character"),
        }
    }
}

impl PuzzleSolution for Day {
    fn part1(&self, puzzle: &Puzzle) -> Answer {
        let grid: Grid<usize, Cell> = puzzle.to_string().into();
        grid.iter()
            .filter(|((x, y), cell)| {
                if *cell != &Cell::Roll {
                    return false;
                }
                let roll_count = grid.all_neighbors(**x, **y)
                    .iter()
                    .filter(|(nx, ny)| grid.get(*nx, *ny) == Some(&Cell::Roll))
                    .count();
                roll_count < 4
            })
            .count()
            .into()
    }

    fn part2(&self, puzzle: &Puzzle) -> Answer {
        let mut grid: Grid<usize, Cell> = puzzle.to_string().into();
        let mut can_remove = true;
        let mut removed = 0;
        while can_remove {
            can_remove = false;
            let ref_grid = grid.clone();
            grid.iter_mut()
                .for_each(|((x, y), cell)| {
                    if *cell != Cell::Roll {
                        return;
                    }
                    let roll_count = ref_grid.all_neighbors(*x, *y)
                        .iter()
                        .filter(|(nx, ny)| ref_grid.get(*nx, *ny) == Some(&Cell::Roll))
                        .count();
                    if roll_count < 4 {
                        *cell = Cell::Empty;
                        removed += 1;
                        can_remove = true;
                    }
                });
        }
        removed.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_puzzle() -> Puzzle {
        Puzzle::from(r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#)
    }

    #[test]
    fn part1() {
        let result = Day::default().part1(&get_puzzle());
        assert_eq!(result, 13.into());
    }

    #[test]
    fn part2() {
        let result = Day::default().part2(&get_puzzle());
        assert_eq!(result, 43.into());
    }
}
