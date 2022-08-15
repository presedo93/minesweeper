use std::{collections::HashSet, fmt::Display, fmt::Write};

use crate::random::random_range;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("💣 ")?;
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟦 ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighbours_mines(pos);

                    if mine_count > 0 {
                        write!(f, " {} ", self.neighbours_mines(pos))?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }

                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }

    pub fn iter_neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        return (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y));
    }

    pub fn neighbours_mines(&self, pos: Position) -> u8 {
        self.iter_neighbours(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<OpenResult> {
        if self.open_fields.contains(&pos) {
            let mine_count = self.neighbours_mines(pos);
            let flag_count = self
                .iter_neighbours(pos)
                .filter(|neighbour| self.flagged_fields.contains(neighbour))
                .count() as u8;

            if mine_count == flag_count {
                for neighbour in self.iter_neighbours(pos) {
                    if !self.flagged_fields.contains(&neighbour)
                        && !self.open_fields.contains(&neighbour)
                    {
                        self.open(neighbour);
                    }
                }
            }
        }

        if self.lost || self.open_fields.contains(&pos) || self.flagged_fields.contains(&pos) {
            return None;
        }

        self.open_fields.insert(pos);

        let is_mine = self.mines.contains(&pos);

        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighbours_mines(pos);

            if mine_count == 0 {
                for neighbour in self.iter_neighbours(pos) {
                    if !self.open_fields.contains(&neighbour) {
                        self.open(neighbour);
                    }
                }
            }

            Some(OpenResult::NoMine(self.neighbours_mines(pos)))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }
        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Minesweeper;

    #[test]
    fn test_minesweeper_new() {
        let mut ms = Minesweeper::new(10, 10, 5);

        ms.open((5, 5));
        ms.toggle_flag((6, 6));

        println!("{}", ms);
    }
}
