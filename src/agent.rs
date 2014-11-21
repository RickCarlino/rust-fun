// The piece can be anything except a tile.
// Ex: Item, Player, NPC
// here we go


pub fn change_position(pos: &mut super::position::Position,
                       delta_x:int,
                       delta_y: int) {
  pos.x = pos.x + delta_x;
  pos.y = pos.y + delta_y;
}

pub struct Agent {
    /// An entity that is capable of action
    pub position: super::position::Position
}

impl Agent {
    pub fn change_position(&mut self, delta_x: int, delta_y: int) {
        println!("Moving, sir!");
        println!("&self.position.x  {}", &self.position.x);
        println!("&self.position.y  {}", &self.position.y);
        self.position.y = *&self.position.y + delta_y;
        self.position.x = *&self.position.x + delta_x;
        println!("I moved");
        println!("&self.position.x  {}", &self.position.x);
        println!("&self.position.y  {}", &self.position.y);
    }

    pub fn new(position: super::position::Position) -> Agent {
        Agent { position: position }
    }
}



