extern crate redis;
use redis::Commands;

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
