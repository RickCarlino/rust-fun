struct Tile {
  pub position: super::position::Position
  pub contains: Vec<Agents>
}


/// board attrs are not mutable.

pub struct Map {
  pub height: int,
  pub width: int,
  pub tiles: Vec<Tile>
}

impl Map {
  pub fn initialize(&self, height: int, width: int) {
    if height < 1 {
      panic!("Height cannot be less than 1");
    }
    if width < 1 {
      panic!("Width cannot be less than 1");
    }
    self.height = height;
    self.width  = width;
  }
}
