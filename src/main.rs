mod position;
mod agent;
mod board;


#[cfg(not(test))]
fn main() {
    let mut some_position = ::position::Position{x:5i, y:4i};
    agent::change_position(&mut some_position, -5i, -4i);
    let mut agent_smith = agent::Agent::new( ::position::Position {x: 1i, y:2i});
    agent_smith.change_position(5i,5i);
}

pub mod tests {

    #[test]
    fn test_position_definition() {
        let some_position = ::position::Position{x:5i, y:4i};
        assert!(some_position.x == 5i);
        assert!(some_position.y == 4i);
    }

    #[test]
    fn test_change_position_fn() {
        let mut some_position = ::position::Position{x:5i, y:4i};
        assert!(some_position.x == 5i);
        assert!(some_position.y == 4i);
        ::agent::change_position(&mut some_position, -5i, -4i);
        assert!(some_position.x == 0);
        assert!(some_position.y == 0);
    }

    #[test]
    fn test_agent_position_defintion() {
      let some_agent = ::agent::Agent::new( ::position::Position {x: 1i, y:2i});
      assert!(some_agent.position.x == 1);
      assert!(some_agent.position.y == 2);
    }

    #[test]
    fn test_agent_change_position() {
      let mut some_agent = ::agent::Agent::new( ::position::Position {x: 1i, y:2i});
      assert!(some_agent.position.x == 1);
      assert!(some_agent.position.y == 2);
      some_agent.change_position(5i,5i);
      assert!(some_agent.position.x == 6);
      assert!(some_agent.position.y == 7);
    }

    #[test]
    fn test_map_new_method() {
      let new_map = ::board::Map::new(4,4); //initialize(5i,4i);
      assert_eq!(new_map.width, 4);
      assert_eq!(new_map.height, 4);
    }

    #[test]
    #[should_fail]
    fn test_map_initialize_not_wide_enough() {
      let new_map = ::board::Map::new(0,4);
    }

    #[test]
    #[should_fail]
    fn test_map_initialize_not_high_enough() {
      let new_map = ::board::Map::new(4,0);
    }
}

