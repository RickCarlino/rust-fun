// The piece can be anything except a tile.
// Ex: Item, Player, NPC
// here we go

struct Agent {
    /// An entity that is capable of action
    position: Position
}

impl Agent {
    fn move(&self) {
        println!("Moving, sir!");
    }
    
    fn new(position: Position) -> Agent {
        Agent { position:position };
    }
}



