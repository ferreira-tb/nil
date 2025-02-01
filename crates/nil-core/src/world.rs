use crate::village::Village;
use std::ops::{Index, IndexMut};

pub struct World {
  cells: Vec<Cell>,
  capacity: usize,
}

impl World {
  pub fn new(size: u8) -> Self {
    let capacity = usize::from(size).pow(2);
    let mut cells = Vec::with_capacity(capacity);
    cells.resize_with(capacity, Cell::default);
    cells.shrink_to_fit();
    Self { cells, capacity }
  }

  fn index(&self, coord: Coord) -> usize {
    let Coord(x, y) = coord;
    usize::from(y) * usize::from(self.capacity) + usize::from(x)
  }

  pub fn get(&self, coord: impl Into<Coord>) -> &Cell {
    let index = self.index(coord.into());
    unsafe { self.cells.get_unchecked(index) }
  }

  pub fn get_mut(&mut self, coord: impl Into<Coord>) -> &mut Cell {
    let index = self.index(coord.into());
    unsafe { self.cells.get_unchecked_mut(index) }
  }
}

impl Index<Coord> for World {
  type Output = Cell;

  fn index(&self, coord: Coord) -> &Self::Output {
    self.get(coord)
  }
}

impl IndexMut<Coord> for World {
  fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
    self.get_mut(coord)
  }
}

#[derive(Default)]
pub enum Cell {
  #[default]
  Empty,
  Village(Village),
}

#[derive(Clone, Copy, Debug)]
pub struct Coord(u8, u8);

impl Coord {
  pub const fn new(x: u8, y: u8) -> Self {
    Self(x, y)
  }
}

impl From<(u8, u8)> for Coord {
  fn from((x, y): (u8, u8)) -> Self {
    Self(x, y)
  }
}
