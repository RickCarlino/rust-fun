extern crate redis;

use redis::Commands;

mod agent;


#[cfg(not(test))]
fn main() {
    let x = fetch_an_integer();
    match x {
        Ok(v) => {
            println!("working with version: {}", v);
        }
        Err(e) => {
            println!("error parsing header: {}", e);
        }
    };
    let mut some_position = agent::Position{x:5i, y:4i};
    println!("I am some_position's x: {}", some_position.x)
    println!("I am some_position's y: {}", some_position.y)

    agent::change_position(&mut some_position, -5i, -4i);

    println!("and after change_position");
    println!("I am some_position's x: {}", some_position.x);
    println!("I am some_position's y: {}", some_position.y);


    let mut agent_smith = agent::Agent::new( agent::Position {x: 1i, y:2i});
    println!("agent_smith's x: {}", agent_smith.position.x);
    println!("agent_smith's y: {}", agent_smith.position.y);

    agent_smith.change_position(5i,5i);

}

fn fetch_an_integer() -> redis::RedisResult<int> {
    // connect to redis
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());
    // throw away the result, just make sure it does not fail
    let _ : () = try!(con.set("my_key", 42i));
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

pub mod tests {

    #[test]
    fn test_fetch_an_integer() {
      let x = super::fetch_an_integer();
      assert!(x == Ok(42i));
    }

    #[test]
    fn test_position_definition() {
        let some_position = ::agent::Position{x:5i, y:4i};
        assert!(some_position.x == 5i);
        assert!(some_position.y == 4i);
    }

    #[test]
    fn test_change_position_fn() {
        let mut some_position = ::agent::Position{x:5i, y:4i};
        assert!(some_position.x == 5i);
        assert!(some_position.y == 4i);
        ::agent::change_position(&mut some_position, -5i, -4i);
        assert!(some_position.x == 0);
        assert!(some_position.y == 0);
    }

    #[test]
    fn test_agent_position_defintion() {
      let some_agent = ::agent::Agent::new( ::agent::Position {x: 1i, y:2i});
      assert!(some_agent.position.x == 1);
      assert!(some_agent.position.y == 2);
    }

    #[test]
    fn test_agent_change_position() {
      let mut some_agent = ::agent::Agent::new( ::agent::Position {x: 1i, y:2i});
      assert!(some_agent.position.x == 1);
      assert!(some_agent.position.y == 2);
      some_agent.change_position(5i,5i);
      assert!(some_agent.position.x == 6);
      assert!(some_agent.position.y == 7);

    }
}

