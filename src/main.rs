extern crate redis;

use redis::Commands;

mod agent;

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
    //agent::change_position()

    println!("and after change_position");

    println!("I am some_position's x: {}", some_position.x);
    println!("I am some_position's y: {}", some_position.y);

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




//objects in rust


