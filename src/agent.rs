// The piece can be anything except a tile.
// Ex: Item, Player, NPC
// here we go

pub struct Position {
  pub x: int,
  pub y: int,
}

pub fn change_position(pos: &mut Position, delta_x:int, delta_y: int) {
  pos.x = pos.x + delta_x;
  pos.y = pos.y + delta_y;
}

pub struct Agent {
    /// An entity that is capable of action
    position: Position
}

impl Agent {
    fn change_position(&self) {
        println!("Moving, sir!");
    }

    fn new(position: Position) -> Agent {
        Agent { position: position }
    }
}



