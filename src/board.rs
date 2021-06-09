use std::fmt;

use anyhow::{bail, Result};

use crate::block::Block;
use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Empty,
    Fill(usize),
    Wall(char),
}

impl State {
    pub fn is_fill(&self) -> bool {
        match self {
            Self::Fill(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            State::Empty => write!(f, "."),
            State::Fill(id) => write!(f, "{}", id),
            State::Wall(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<Vec<State>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height() {
            for j in 0..self.width() {
                write!(f, "{} ", self.board[i][j])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new_with_walls(height: usize, width: usize, walls: &[(char, Point)]) -> Self {
        let mut board = vec![vec![State::Empty; width]; height];
        for (c, p) in walls {
            board[p.x as usize][p.y as usize] = State::Wall(*c);
        }
        Self { board }
    }

    pub fn height(&self) -> usize {
        self.board.len()
    }

    pub fn width(&self) -> usize {
        self.board[0].len()
    }

    pub fn put_block(&mut self, pos: &Point, block_id: usize, block: &Block) -> Result<()> {
        let ps: Vec<Point> = block.ps.iter().map(|p| *pos + *p).collect();
        if !ps
            .iter()
            .all(|p| self.get_cell(p).map(|s| s == State::Empty).unwrap_or(false))
        {
            bail!("cannot put");
        }

        for p in ps {
            self.board[p.x as usize][p.y as usize] = State::Fill(block_id);
        }
        Ok(())
    }

    pub fn remove_block(&mut self, pos: &Point, block: &Block) -> Result<()> {
        let ps: Vec<Point> = block.ps.iter().map(|p| *pos + *p).collect();
        if !ps
            .iter()
            .all(|p| self.get_cell(p).map(|s| s.is_fill()).unwrap_or(false))
        {
            bail!("cannot remove");
        }

        for p in ps {
            self.board[p.x as usize][p.y as usize] = State::Empty;
        }
        Ok(())
    }

    pub fn get_cell(&self, p: &Point) -> Result<State> {
        if !(0 <= p.x
            && (p.x as usize) < self.height()
            && 0 <= p.y
            && (p.y as usize) < self.width())
        {
            bail!("invalid cell: {}", p);
        }
        Ok(self.board[p.x as usize][p.y as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill() {
        let mut board = Board::new(2, 2);

        #[rustfmt::skip]
        let v = vec![
            "#.",
            ".#",
        ];

        let b = Block::from_strs(&v).unwrap();
        let b1 = b.rot();

        board.put_block(&Point::new(0, 0), &b).unwrap();
        board.put_block(&Point::new(1, 0), &b1).unwrap();

        for l in board.board {
            for s in l {
                assert_eq!(s, State::Fill);
            }
        }
    }

    #[test]
    fn test_overlap() {
        let mut board = Board::new(2, 2);

        #[rustfmt::skip]
        let v = vec![
            "#.",
            ".#"
        ];

        let b = Block::from_strs(&v).unwrap();

        board.put_block(&Point::new(0, 0), &b).unwrap();
        assert!(board.put_block(&Point::new(0, 0), &b).is_err());
    }
}
