use agent;


/// board attrs are not mutable.

pub struct Map {
  pub height: int,
  pub width: int,
  //pub tiles: Vec<Tile>,
}

impl Map {
  pub fn new(height: int, width: int )-> Map {
    if height <= 1 {
      panic!("Height cannot be less than 1");
    }
    if width <= 1 {
      panic!("Width cannot be less than 1");
    }
    return Map {height: height, width: width}
  }

  pub fn within(&self, height: int, width: int) -> bool {
    return height <= self.height && width <= self.width
  }
}
