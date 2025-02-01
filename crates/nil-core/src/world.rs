#![allow(dead_code)]

pub struct World {
  size: u8,
  cells: Vec<Cell>,
}

impl World {
  pub fn new(size: u8) -> Self {
    let capacity = usize::from(size) * usize::from(size);
    let mut cells = Vec::with_capacity(capacity);
    cells.resize_with(capacity, Cell::default);
    cells.shrink_to_fit();
    Self { size, cells }
  }
}

#[derive(Default)]
pub enum Cell {
  #[default]
  Empty,
  Village,
}
